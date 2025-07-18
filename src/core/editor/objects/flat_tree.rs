use std::cmp::Ordering;
use std::fmt;

use egui::ahash::{HashMap, HashMapExt};

use crate::utils::error::ErrorType;
use crate::utils::{error::EditorIoError, io::FileEntry};
use crate::utils::io;
use crate::EDITOR_ROOT_DIR;


///TreeNode structure that exists inside FlatTree.
///     - Id: Depth * 10 + offset
///     - Depth: Starts at 1..
#[derive(Clone)]
pub(crate) struct TreeNode{
    pub(crate) id: usize,
    pub(crate) depth: usize,
    pub(crate) file_entry: FileEntry,
    children: Vec<usize>,
    parent: usize,
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
    elements: Vec<TreeNode>
}
impl FlatTree{
    fn new() -> FlatTree{
        return FlatTree{
            elements: Vec::new()
        }
    }
    
    ///Builds a layer of the FlatTree. This implementation avoids recursion, and must therefore
    /// be used in a controlled loop to successfully build a whole and complete directory tree.
    fn build(&mut self, directory: &Vec<FileEntry>){
        let mut offset: usize = 1;

        if self.elements.is_empty(){
            for file_entry in directory{
                self.add_as_root(file_entry, offset);
                offset += 1;
            }
        }
        else{
            for element in directory{
                let parent_name = element.parent.clone();

                if let Some((parent_index, parent_id, parent_depth)) = self.get_parent(parent_name.as_str()){
                    let id = parent_id * 10 + offset;

                    self.add_as_child(element, parent_id, parent_depth, id);
                    
                    if let Some(parent) = self.elements.get_mut(parent_index){
                        if !parent.children.contains(&id){
                            parent.children.push(id);
                        }
                    }
                }
                offset += 1;
            }
        }
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
        self.elements.push(new_node.clone());
    }

    ///Returns the parents index, id and depth based on entry *name*, if exists.
    fn get_parent(&self, name: &str) -> Option<(usize, usize, usize)>{
        self.elements
            .iter()
            .enumerate()
            .find(|entry| entry.1.file_entry.name == name)
            .map(|(index, node)| (index, node.id, node.depth))
    }

    ///Creates and returns a new Flat Tree structure that is prepared (ordered and sorted) 
    /// to show in UI context.
    pub(crate) fn get_visible_items(&mut self) -> Vec<&TreeNode>{
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
            
            //Review: Don't break, in case of empty dir
            if parent_groups.is_empty() {
                continue;
            }
            //Sort based on TreeNodes
            for items in parent_groups.values_mut() {
                items.sort();
            }
            
            // Process each parent group
            for (parent_id, items) in parent_groups {
                //Non-root item
                if parent_id != 0{
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
                //Root level items are always added
                else{
                    structure.extend(items);
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
    pub(crate) fn init() -> Result<TreeBuilder, EditorIoError>{
        let editor_dir_path= EDITOR_ROOT_DIR.get().unwrap();
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
        let current_entries = &self.current.take();

        if let Some(current_directory) = current_entries{
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