use std::{collections::HashSet, path::PathBuf};

use egui::{DroppedFile, Pos2, Ui};
use egui_tiles::{TileId, Tree};

use crate::{core::editor::objects::{settings::{EditorSettings, FileTreeSettings}, ui_tree::{create_tree, Pane, PaneKind, PaneType, TreeBehavior}}, utils::error::EditorIoError};


///This struct is an abstraction over Tiles/Panes and general
/// UI handling of lower level UI entities.
#[derive(Clone)]
pub(crate) struct EditorLayout{
    tree: Tree<Pane>,
    pub(crate) dropped_files: Vec<PathBuf>
}
impl EditorLayout{
    pub(crate) fn new(editor_settings: EditorSettings) -> Result<EditorLayout, EditorIoError>{
        match create_tree(editor_settings){
            Ok(tree) => {
                return Ok(EditorLayout{
                    tree: tree,
                    dropped_files: Vec::new()
                })
            },
            Err(err) => return Err(err),
        }
    }

    ///Middleware abstraction for UI tree rendering.
    pub(crate) fn ui(&mut self, ui: &mut Ui){
        let mut behavior = TreeBehavior{};
        self.tree.ui(&mut behavior, ui);
    }

    ///Reloads panes with new settings provided by menu.
    /// Requires a double iteration since Tile/Pane visibility/mutation is a bit weird.
    pub(crate) fn reload(&mut self, ui_changes: Vec<PaneKind>, settings: &EditorSettings){
        let changed_panes: HashSet<PaneKind> = ui_changes.into_iter().collect();
       
        // First pass: collect tile id's of containers that 
        // 1) Are panes, 2) Match PaneKind
        let tile_ids_to_update: Vec<TileId> = self.tree
            .tiles
            .iter()
            .filter_map(|(tile_id, tile)| {
                if let egui_tiles::Tile::Pane(inner_pane) = tile {
                    if changed_panes.contains(&inner_pane.get_kind()) {
                        Some(*tile_id)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
       
        // Second pass: mutate the panes
        for tile_id in tile_ids_to_update {
            if let Some(tile) = self.tree.tiles.get_mut(tile_id) {
                match tile{
                    egui_tiles::Tile::Pane(pane) => {
                        match pane.get_type(){
                            PaneType::FileTree { .. } => {
                                let new_settings = FileTreeSettings::from(settings.clone());

                                pane.reload_with_settings(new_settings);
                            },
                            PaneType::Inspector { variables, new_key, new_value } => todo!(),
                            PaneType::Console { messages, input } => todo!(),
                            PaneType::Empty => todo!(),
                        }
                    },
                    _ => {}
                }
                
            }
        }
    }

    
    pub(crate) fn file_hovered(&mut self, file: PathBuf){
        self.dropped_files.push(file.to_path_buf());
    }

    pub(crate) fn clear_dropped_list(&mut self){
        self.dropped_files.clear();
    }

    #[deprecated(note="WINIT/EGUI does not *currently* handle file `dnd` from external sources.")]
    pub(crate) fn handle_file_drop(&mut self, drop_pos: &Option<Pos2>){
        match drop_pos{
            Some(drop) => {
                let files = self.dropped_files.clone();
        
                let tiles = &self.tree.tiles.clone();
                //Iterate tile ids
                for tid in tiles.tile_ids(){
                    //If tile rect contains mouse pointer
                    if tiles.rect(tid).is_some_and(|rect| rect.contains(*drop)){
                        if let Some(tile) = self.tree.tiles.get_mut(tid){
                            match tile{
                                egui_tiles::Tile::Pane(ref mut pane) => {
                                    for file in &files{
                                        pane.file_dropped(file);
                                    }
                                },
                                egui_tiles::Tile::Container(_) => {
                                    continue;
                                },
                            }
                        }
                    }
                }

                self.dropped_files.clear();
            },
            None => {
                eprintln!("Error: No drop position acquired.");
            },
        }
        

    }
}



