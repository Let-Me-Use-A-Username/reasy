use std::cmp::Ordering;
use std::fmt;

use crate::utils::error::ErrorType;
use crate::utils::{error::EditorIoError, io::FileEntry};
use crate::utils::io;
use crate::PROJECT_ROOT_DIR;


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

    ///Returns the parents index, id and depth if exists.
    fn get_parent(&self, name: &str) -> Option<(usize, usize, usize)>{
        self.elements
            .iter()
            .enumerate()
            .find(|entry| entry.1.file_entry.name == name)
            .map(|(index, node)| (index, node.id, node.depth))
    }

    pub(crate) fn get_visible_items(&mut self) -> Vec<&TreeNode>{
        let mut structure: Vec<&TreeNode> = Vec::new();

        //Get max depth
        let max_depth = self.elements
            .iter()
            .map(|entry| entry.depth)
            .max()
            .unwrap_or(0);

        //Iterate entries by depth
        for depth in 0..max_depth{
            let mut depth_items = self.elements
                .iter()
                .filter(|entry| {
                    //Collect visible entries
                    entry.depth == depth
                    &&
                    entry.visible
                })
                .collect::<Vec<&TreeNode>>();
            
            if depth_items.is_empty(){
                break;
            }
            
            depth_items.sort(); 
            //FIXME: When multiple directories open in same depth, false parent
            //FIXME: is provided, therefore grouping different directories elements together
            let parent_node = depth_items.first().unwrap();

            //Find parent position in ordered vector
            if let Some(ref mut parent_pos) = structure
                .iter()
                // .inspect(|entry| {
                //     println!("Looking for parent: {:?} in node: {:?}", parent_node.file_entry.parent, entry.file_entry.name);
                //     println!("Looking for parent id : {:?} in node with id : {:?}", parent_node.parent, entry.id);
                // })
                .position(|element| element.id == parent_node.parent)
            {
                *parent_pos += 1;
                for item in depth_items.iter().rev(){
                    structure.insert(*parent_pos, item);
                }
            }
            //No parent found
            else{
                structure.extend(depth_items);
            }
            
        }
        
        return structure
    }

    pub(crate) fn toggle_visibility(&mut self, id: &usize) {
        if let Some(node) = self.elements.iter_mut().find(|entry| entry.id == *id) {
            let is_visible = node.visible;
            let is_expanded = node.expanded;
            let children_id = node.children.clone();

            //Hide children
            if is_visible && is_expanded {
                node.expanded = false;
                self.toggle_children(&children_id, Some(false));
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

    fn toggle_children(&mut self, children: &Vec<usize>, force_visibility: Option<bool>) {
        for child_id in children {
            if let Some(node) = self.elements.iter_mut().find(|entry| entry.id == *child_id) {
                let visibility = force_visibility.unwrap_or(!node.visible);
                node.visible = visibility;
            }
        }
    }
}




///Helper struct to build FlatTree structure layer by layer. 
/// Where layer here is a certain depth of a file system.
pub(crate) struct TreeBuilder{
    current: Option<Vec<FileEntry>>,
    next: Option<Vec<FileEntry>>,
    tree: FlatTree
}
impl TreeBuilder{
    pub(crate) fn init() -> Result<TreeBuilder, EditorIoError>{
        let current_dir_path= PROJECT_ROOT_DIR.get().unwrap();
        let current_directory = io::read_directory(current_dir_path.as_path());
        
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

        if let Some(next_directory) = self.next.take(){
            for next_item in next_directory {
                if next_item.is_dir{
                    let directory = io::read_directory(&next_item.path)?;
                    
                    for entry in directory {
                        // Collect subdirectories for the next level
                        if entry.is_dir {
                            dirs_for_next_level.push(entry.clone());
                        }
                        next_items.push(entry);
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

    pub(crate) fn get_tree(&mut self) -> FlatTree{
        return self.tree.clone()
    }
}