use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hasher};
use std::sync::Arc;
use std::{env, fmt};
use std::path::{Path, PathBuf};
use std::hash::Hash;

use crate::utils::error::ErrorType;
use crate::utils::{error::EditorIoError, io::FileEntry};
use crate::utils::io;
use crate::EDITOR_ROOT_DIR;


///TreeNode structure that exists inside FlatTree.
#[derive(Clone)]
pub(crate) struct TreeNode{
    pub(crate) id: usize,
    pub(crate) depth: usize,
    pub(crate) file_entry: FileEntry,
    children: Vec<usize>,
    pub(crate) parent: usize,
    pub(crate) visible: bool,
    pub(crate) expanded: bool
}

impl fmt::Debug for TreeNode{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeNode")
            //.field("id", &self.id)
            //.field("depth", &self.depth)
            //.field("entry", &self.entry)
            .field("name", &self.file_entry.name)
            //.field("children", &self.children)
            //.field("parent", &self.parent)
            //.field("visible", &self.visible)
            //.field("expanded", &self.expanded)
            .finish()
    }
}

impl PartialOrd for TreeNode{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TreeNode{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.file_entry.is_dir, other.file_entry.is_dir) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => self.file_entry.name.cmp(&other.file_entry.name)
        }
    }
}

impl PartialEq for TreeNode{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.depth == other.depth
    }
}

impl Eq for TreeNode{}




///FlatTree structure used in UI file tree.
#[derive(Debug, Clone)]
pub(crate) struct FlatTree{
    elements: Vec<TreeNode>,
    lookup: HashMap<usize, usize>
}
impl FlatTree{
    fn new() -> FlatTree{
        return FlatTree{
            elements: Vec::new(),
            lookup: HashMap::new()
        }
    }

    //Retrieve a mutable node reference via node id.
    fn get_node_mut(&mut self, id: usize) -> Option<&mut TreeNode> {
        if let Some(node_index) = self.lookup.get(&id){
            return self.elements.get_mut(*node_index)
        }
        else{
            None
        }
    }
    
    //Retrieve a node reference via node id.
    fn get_node(&self, id: usize) -> Option<&TreeNode> {
        if let Some(node_index) = self.lookup.get(&id){
            return self.elements.get(*node_index)
        }
        else{
            None
        }
    }

    ///Rebuils lookup table
    fn rebuild_index(&mut self){
        self.lookup.clear();

        for (index, node) in self.elements.iter().enumerate(){
            self.lookup.insert(node.id, index);
        }
    }
    
    ///Builds a layer of the FlatTree. This implementation avoids recursion, and must therefore
    /// be used in a controlled loop to successfully build a whole and complete directory tree.
    fn build(&mut self, directory: &Vec<FileEntry>){
        if self.elements.is_empty(){
            for file_entry in directory{
                let id = Self::path_to_id(&file_entry.path);
                self.add_as_root(file_entry, id);
            }
        }
        else{
            for element in directory{
                let id = Self::path_to_id(&element.path);

                if self.lookup.contains_key(&id){
                    continue;
                }

                let mut parent_path = PathBuf::from(element.path.clone());
                parent_path.pop();

                if let Some((_, parent_id, parent_depth)) = self.get_parent_from_path(&parent_path) {
                    self.add_as_child(element, parent_id, parent_depth, id);
                    
                    // Add child to parent's children list
                    if let Some(parent) = self.get_node_mut(parent_id) {
                        if !parent.children.contains(&id){
                            parent.children.push(id);
                        }
                    }
                }
            }
        }

        self.rebuild_index();
    }

    ///Add entry as child to some element.
    fn add_as_child(&mut self, element: &FileEntry, pid: usize, pdepth: usize, id: usize){
        let new_node = TreeNode{
            id: id,
            depth: pdepth + 1,
            file_entry: element.clone(),
            children: vec![],
            parent: pid,
            visible: false,
            expanded: false,
        };
        self.elements.push(new_node);
        self.lookup.insert(id, self.elements.len());
    }

    ///Add entry as root.
    fn add_as_root(&mut self, element: &FileEntry, id: usize){
        let new_node = TreeNode { 
            id: id, 
            depth: 0, 
            file_entry: element.clone(), 
            children: vec![], 
            parent: 0, 
            visible: true,
            expanded: false,
        };
        self.elements.push(new_node);
        self.lookup.insert(id, self.elements.len());
    }

    ///Returns the parents index, id and depth based on entry *path*, if exists.
    fn get_parent_from_path(&self, path: &PathBuf) -> Option<(usize, usize, usize)>{
        self.elements
            .iter()
            .enumerate()
            .find(|(_, node)| node.file_entry.path == *path)
            .map(|(index, node)| (index, node.id, node.depth))
    }

    ///Creates and returns a new Flat Tree structure that is prepared (ordered and sorted) 
    /// to show in UI context.
    pub(crate) fn get_visible_items(&self) -> Vec<&TreeNode>{
        let mut structure: Vec<&TreeNode> = Vec::new();

        //Get max depth
        let max_depth = self.elements
            .iter()
            .map(|entry| entry.depth)
            .max()
            .unwrap_or(0);

        //Iterate entries by depth
        for depth in 0..=max_depth{
            // Group items by parent id
            let mut parent_groups: HashMap<usize, Vec<&TreeNode>> = HashMap::new();
            
            // Collect all visible items at this depth and group them by parent
            for node in self.elements.iter().filter(|entry| entry.depth == depth && entry.visible) {
                parent_groups.entry(node.parent).or_insert_with(Vec::new).push(node);
            }
            
            //Note: Don't break, in case of empty dir
            if parent_groups.is_empty() {
                continue;
            }
            //Sort based on TreeNodes
            for items in parent_groups.values_mut() {
                items.sort();
            }
            
            // Process each parent group
            for (parent_id, items) in parent_groups {
                //Root level items (parent_id == 0) are always added
                if parent_id == 0{
                    structure.extend(items);
                }
                //Non-root item
                else {
                    //Check if parent is expanded
                    let parent_expanded = self.get_node(parent_id)
                        .map(|node| node.expanded)
                        .unwrap_or(false);
                    
                    //If not expanded, skip these children
                    if !parent_expanded{
                        continue;
                    }

                    // Default placement position
                    let mut insert_pos = structure.len();
                    
                    // Find the parent position and assign the next spot
                    if let Some(parent_pos) = structure.iter().position(|element| element.id == parent_id) {
                        insert_pos = parent_pos + 1;
                        
                        // Find the last child of this parent that's already in the structure
                        // Append after it to maintain an order... older -> newer
                        for i in (parent_pos + 1)..structure.len() {
                            if structure[i].parent == parent_id {
                                insert_pos = i + 1;
                            } 
                            else {
                                break;
                            }
                        }
                    }
                    
                    // Insert items in order (regarding parents structure)
                    for (i, item) in items.iter().enumerate() {
                        structure.insert(insert_pos + i, item);
                    }
                }
            }
        }
        
        structure
    }

    //Toggle visibility of a directory.
    pub(crate) fn toggle_visibility(&mut self, id: &usize) {
        if let Some(node) = self.elements.iter_mut().find(|entry| entry.id == *id) {
            let is_visible = node.visible;
            let is_expanded = node.expanded;
            let children_id = node.children.clone();
            
            //Hide children
            if is_visible && is_expanded {
                node.expanded = false;
                self.toggle_children(&children_id, Some(false));
                self.toggle_expantion(&children_id, Some(false));
            }
            //Show children
            else if is_visible && !is_expanded {
                node.expanded = true;
                self.toggle_children(&children_id, Some(true));
            } 
            //Hide self and children
            else {
                node.visible = false;
                node.expanded = false;
                self.toggle_children(&children_id, Some(false));
            }
        }
    }

    //Toggle or force expanded on children.
    fn toggle_expantion(&mut self, children: &Vec<usize>, force_expantion: Option<bool>){
        for child_id in children {
            if let Some(node) = self.elements.iter_mut().find(|entry| entry.id == *child_id) {
                let expanded = force_expantion.unwrap_or(!node.visible);
                node.expanded = expanded;
            }
        }
    }

    ///Toggle or force visibility on children.
    fn toggle_children(&mut self, children: &Vec<usize>, force_visibility: Option<bool>) {
        for child_id in children {
            if let Some(node) = self.elements.iter_mut().find(|entry| entry.id == *child_id) {
                let visibility = force_visibility.unwrap_or(!node.visible);
                node.visible = visibility;

                if !visibility {
                    let grandchildren = node.children.clone();
                    self.toggle_children(&grandchildren, Some(false));
                }
            }
        }
    }

    ///Create a sub-section of FlatTree from nodes.
    /// Used to retrieve the 'visible' UIDirectory tree.
    pub(crate) fn get_children_from_ids(&mut self, children: &Vec<usize>) -> Vec<&TreeNode>{
        let mut returned = Vec::new();

        for child_id in children{
            if let Some(child) = self.elements.iter().find(|entry| entry.id == *child_id){
                returned.push(child);
            }
        }

        return returned;
    }

    ///Takes a node's path and returns a hashed id.
    fn path_to_id(path: &PathBuf) -> usize{
        let mut hasher = DefaultHasher::new();
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());
        canonical_path.hash(&mut hasher);

        return hasher.finish() as usize
    }

    ///Removes a node based on ID.
    pub(crate) fn remove(&mut self, id: usize) -> Option<TreeNode>{
        if let Some(node_pos) = self.lookup.get(&id){
            let node = self.elements.remove(*node_pos);

            if node.file_entry.is_dir{
                for child in &node.children{
                    self.remove(*child);
                }
            }
            self.rebuild_index();

            return Some(node)
        }

        return None
    }

    ///Renames a node, also changes its path, and if directory changes children parent path and their path.
    pub(crate) fn rename(&mut self, id: usize, new_name: &String) -> Option<String>{
        let mut nodes_to_change = Vec::new();
        let mut new_parent_path = None;
        let mut old_name = None;

        if let Some(node_pos) = self.lookup.get(&id){
            if let Some(node) = self.elements.get_mut(*node_pos){
                //Change node's name and path
                let new_path = Path::new(&node.file_entry.parent).join(&new_name);
                old_name = Some(node.file_entry.path.display().to_string());
                node.file_entry.name = new_name.to_string();
                node.file_entry.path = new_path.clone();

                if node.file_entry.is_dir{
                    for child in node.children.clone(){
                        nodes_to_change.push(child);
                    }
                    new_parent_path = Some(new_path);
                }
            }
        }

        //If node is dir, change children parent and path.
        if let Some(path) = new_parent_path{
            for child in nodes_to_change{
                self.rename_parent(child, &path);
            }
        }

        return old_name
    }

    ///Renames a child component who's parent has been renamed.
    fn rename_parent(&mut self, id: usize, new_parent_path: &PathBuf){
        if let Some(node_pos) = self.lookup.get(&id){
            if let Some(node) = self.elements.get_mut(*node_pos){
                let new_path = Path::new(&new_parent_path).join(&node.file_entry.name);
                //Assign new child path and parent path
                node.file_entry.path = new_path.clone();
                node.file_entry.parent = new_parent_path.display().to_string();
            }
        }
    }
}




///Helper struct to build FlatTree structure layer by layer. 
/// Helper contains `current` and `next` layers for building.
/// Each of those are the Items of 1..N directory items.
pub(crate) struct TreeBuilder{
    current: Option<Vec<FileEntry>>,
    next: Option<Vec<FileEntry>>,
    tree: FlatTree,
}
impl TreeBuilder{
    pub(crate) fn init(path: Option<PathBuf>) -> Result<TreeBuilder, EditorIoError>{
        let editor_dir_path = {
            //If path provided
            if path.is_some(){
                path.unwrap()
            }
            //Else default back to EDITOR ROOT
            else{
                let path = EDITOR_ROOT_DIR.get();
                path.unwrap_or(&Arc::new(PathBuf::from(env::current_dir().unwrap()))).to_path_buf()
            }
        };
        let current_directory = io::read_directory(
            editor_dir_path.as_path()
        );
        
        match current_directory{
            Ok(dir) => {
                let nested_dirs = dir
                    .iter()
                    .filter(|entry| entry.is_dir)
                    .map(|entry| entry.clone())
                    .collect::<Vec<FileEntry>>();
                
                return Ok(TreeBuilder { 
                    current: Some(dir), 
                    next: Some(nested_dirs),
                    tree: FlatTree::new()
                })
            },
            Err(err) => return Err(err.into()),
        }
    }

    ///Build tree layer from current items.
    fn build_tree_layer(&mut self) -> bool{
        if let Some(current_directory) = &self.current.take(){
            self.tree.build(current_directory);
            return true
        }
        else{
            //Finished building tree.
            return false
        }
    }

    ///Take `next` items and assign them to current.
    fn get_next(&mut self) -> Result<(), EditorIoError>{
        if self.current.is_some(){
            return Err(EditorIoError::new("Overwriting file entries", ErrorType::Interrupted))
        }

        let mut next_items = Vec::new();
        let mut dirs_for_next_level = Vec::new();

        //Get next targets (all dirs)
        if let Some(next_directory) = self.next.take(){
            //For item in dir
            for next_item in next_directory {
                if next_item.is_dir{
                    let directory = io::read_directory(
                        &next_item.path
                    )?;
                    
                    for entry in directory {
                        // Collect subdirectories for the next level
                        if entry.is_dir {
                            dirs_for_next_level.push(entry.clone());
                        }
                        next_items.push(entry.clone());
                    }
                }
            }
        }
        
        // Set up for next iteration
        self.current = if next_items.is_empty() {
            None
        } else {
            Some(next_items)
        };
        
        self.next = if dirs_for_next_level.is_empty() {
            None
        } else {
            Some(dirs_for_next_level)
        };

        return Ok(())
    }

    ///Builds a FlatTree vector, required before retrieving Tree.
    pub(crate) fn build(&mut self) -> Result<(), EditorIoError>{
        while self.build_tree_layer(){
            let next = self.get_next();

            if next.is_err(){
                return Err(next.unwrap_err().into())
            }

            if self.current.is_none(){
                break;
            }
        }

        return Ok(())
    }

    ///Retrieve Flat Tree.
    pub(crate) fn get_tree(&mut self) -> FlatTree{
        return self.tree.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /* 
        Creates a directory and validates
            1. Number of nodes in tree
            2. Specific nodes exist in tree
            3. Number of nodes in different depths
    */
    #[test]
    fn test_flat_tree_build() {
        //test_data/file1
        //test_data/file2.txt
        let test_dir = "test_data";
        let file_1 = format!("{}/file1", test_dir);
        let file_2 = format!("{}/file2.txt", test_dir);

        //test_data/sub/file1.txt
        //test_data/sub/file2
        let sub_dir = format!("{}/sub", test_dir);
        let sub_file_1 = format!("{}/file1.txt", sub_dir);
        let sub_file_2 = format!("{}/file2", sub_dir);

        //test_data/sub/sub/fil1.txt
        //test_data/sub/sub/file2
        let sub_sub_dir = format!("{}/sub", sub_dir);
        let sub_sub_file_1 = format!("{}/file1.txt", sub_sub_dir);
        let sub_sub_file_2 = format!("{}/file2", sub_sub_dir);

        fs::create_dir_all(&test_dir).unwrap();
        fs::write(&file_1, "test1").unwrap();
        fs::write(&file_2, "test2").unwrap();

        fs::create_dir_all(&sub_dir).unwrap();
        fs::write(&sub_file_1, "test1").unwrap();
        fs::write(&sub_file_2, "test2").unwrap();

        fs::create_dir_all(&sub_sub_dir).unwrap();
        fs::write(&sub_sub_file_1, "test1").unwrap();
        fs::write(&sub_sub_file_2, "test2").unwrap();

        let mut builder = TreeBuilder::init(Some(PathBuf::from(test_dir))).unwrap();
        let _ = builder.build();

        let tree = builder.get_tree();

        assert!(!tree.elements.is_empty(), "Tree should not be empty");
        assert!(tree.elements.iter().any(|n| n.file_entry.name == "file1"));
        assert!(tree.elements.iter().any(|n| n.file_entry.name == "file2.txt"));
        assert_eq!(tree.elements.len(), 8);

        let mut depth = 0;
        
        loop{
            let current_depth_nodes = tree.elements.iter()
                .filter(|node| node.depth == depth)
                .count();

            match depth{
                0 => {
                    assert_eq!(current_depth_nodes, 3);
                },
                1 => {
                    assert_eq!(current_depth_nodes, 3);
                },
                2 => {
                    assert_eq!(current_depth_nodes, 2);
                }
                _ => break
            }

            depth += 1;
        }

        // Cleanup
        fs::remove_dir_all(test_dir).unwrap();
    }


    /* 
        Builds a tree in the current working dir and validate that only one
        instance of each child exists per node.
    */
    #[test]
    fn test_node_duplicate_children() {
        let mut builder = TreeBuilder::init(Some(PathBuf::from("."))).unwrap();
        let _ = builder.build();
        let tree = builder.get_tree();

        // Check that no node has duplicate children
        for node in &tree.elements {
            let mut child_set = std::collections::HashSet::new();
            for &child_id in &node.children {
                assert!(child_set.insert(child_id), 
                    "Node {} has duplicate child {}", node.id, child_id);
            }
        }
    }
    

    /* 
        Builds a tree and validates that a node exists only once in the whole tree.
    */
    #[test]
    fn test_node_duplicates() {
        let mut builder = TreeBuilder::init(Some(PathBuf::from("."))).unwrap();
        let _ = builder.build();
        let tree = builder.get_tree();
        let tree_items = tree.elements.clone();

        for element in tree_items{

            let parent_instances = tree.elements.iter()
                .filter(|node| node.id == element.id)
                .map(|node| node.id)
                .collect::<Vec<usize>>();

            //Assets every node exists only once
            assert_eq!(parent_instances.len(), 1);
        }
    }

}