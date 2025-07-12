use crate::utils::error::ErrorType;
use crate::utils::{error::EditorIoError, io::FileEntry};
use crate::utils::io;
use crate::PROJECT_ROOT_DIR;


/*
    Depth starts from 0 and goes to infinity.
    Id is depth * 10 + offset. 
*/
#[derive(Debug, Clone)]
pub(crate) struct TreeNode{
    pub(crate) id: usize,
    depth: usize,
    pub(crate) entry: FileEntry,
    children: Vec<usize>,
    parent: usize,
    pub(crate) visible: bool,
    pub(crate) expanded: bool
}


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
            for entry in directory{
                self.add_as_root(entry, offset);
                offset += 1;
            }
        }
        else{
            for element in directory{
                let parent_path = element.parent.clone();
                //println!("Entry : {:?}, with cataloged parent: {:?}", element.clone(), element.parent);

                if let Some((parent_index, parent_id, parent_depth)) = self.get_parent(parent_path.as_str()){
                    let id = parent_id * 10 + offset;
                    //println!("Found parent");

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
            entry: element.clone(),
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
            entry: element.clone(), 
            children: vec![], 
            parent: 0, 
            visible: true,
            expanded: false,
        };
        self.elements.push(new_node);
    }

    ///Returns the parents index, id and depth if exists.
    fn get_parent(&self, path: &str) -> Option<(usize, usize, usize)>{
        self.elements
            .iter()
            .enumerate()
            // .inspect(|entry| {
            //     println!("Node: {:?} parent: {:?} path: {:?}", entry.1.entry.name, entry.1.entry.parent, entry.1.entry.path);
            //     println!("Looking for parent: {:?}", path);
            // })
            .find(|entry| entry.1.entry.name == path)
            .map(|(index, node)| (index, node.id, node.depth))
    }

    pub(crate) fn get_visible_items(&mut self) -> Vec<&TreeNode>{
        let visible_items = self.elements
            .iter()
            // .inspect(|entry| {
            //     println!("Entry: {:?} is visible: {:?}", entry.entry.name, entry.visible);
            // })
            .filter(|entry| {
                entry.depth == 0 || entry.visible == true
            })
            .collect::<Vec<&TreeNode>>();
        
        return visible_items
    }

    pub(crate) fn toggle_visibility(&mut self, id: &usize) {
        if let Some(node) = self.elements.iter_mut().find(|entry| entry.id == *id) {
            let is_visible = node.visible;
            let is_expanded = node.expanded;
            let children_id = node.children.clone();

            if is_visible && is_expanded {
                node.expanded = false;
                self.toggle_children(&children_id, Some(false));
            } 
            else if is_visible && !is_expanded {
                node.expanded = true;
                self.toggle_children(&children_id, Some(true));
            } else {
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