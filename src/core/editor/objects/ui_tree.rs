use core::f32;
use std::{collections::{HashMap, HashSet}, os::windows::fs::MetadataExt, path::PathBuf};

use egui_tiles::{Tiles, Tree, UiResponse};

use crate::{core::editor::objects::{flat_tree::{FlatTree, TreeBuilder}, settings::{EditorSettings, FileTreeSettings}}, utils::error::EditorIoError};



///Component used to track and render file systems via a FlatTree structure.
/// Keeps the whole tree in `flat_tree` and a cached version of the visible 
/// nodes in `display_tree`.
/// 
/// Recomputes the tree to be displayed in every frame via the cached `display_tree`.
#[derive(Debug, Clone)]
pub(crate) struct UiDirectory{
    flat_tree: FlatTree,
    display_tree: Vec<usize>
}
impl UiDirectory{
    pub(crate) fn reload(&mut self, show_hidden: bool){
        let visible_nodes = self.flat_tree.get_visible_items();
        
        let mut dirs_to_collapse = Vec::new();
        
        //First pass for dir collection
        for node in visible_nodes.iter() {
            let is_hidden = node.file_entry.metadata.file_attributes() & 0x2 != 0;
            let show = show_hidden || !is_hidden;
            
            //If node is dir, is expanded and shouldn't be visible, toggle it
            if !show && node.file_entry.is_dir && node.expanded {
                dirs_to_collapse.push(node.id);
            }
        }
        
        //Toggle directories that should be hidden
        for dir_id in dirs_to_collapse {
            self.flat_tree.toggle_visibility(&dir_id);
        }

        //Collect visible nodes
        let mut display_tree = Vec::new();
        let mut visible_parent_ids = HashSet::new();

        //Note: Dirs have been collapsed, these items are different from the first ones.
        let updated_visible_nodes = self.flat_tree.get_visible_items();
        
        for node in updated_visible_nodes.iter() {
            
            //Add all root items
            if node.depth == 0 {
                display_tree.push(node.id);
                visible_parent_ids.insert(node.id);
            }
            //Add children items that have a visible parent
            else if visible_parent_ids.contains(&node.parent) {
                display_tree.push(node.id);
                visible_parent_ids.insert(node.id);
            }
        }
        
        self.display_tree = display_tree;
    }
}

///Enum used mainly by the Layout-Menu handlers
/// to communicate what panes needs reloading.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub(crate) enum PaneKind{
    FileTree,
    Inspector,
    Console,
    Empty
}

//Enum used mainly for creation/modification of panes.
#[derive(Debug, Clone)]
pub(crate) enum PaneType{
    FileTree{
        directory: UiDirectory,
        settings: FileTreeSettings
    },
    Inspector{
        variables: HashMap<String, String>,
        new_key: String,
        new_value: String,
    },
    Console{
        messages: Vec<String>,
        input: String
    },
    Empty
}

///Pane object that resides inside a egui_tiles Tile.
/// Contains the UI the user interacts with and is regarded 
/// as a leaf entity inside the UI tree.
#[derive(Clone)]
pub(crate) struct Pane {
    _id: usize, 
    pane_type: PaneType,
    title: String,
    kind: PaneKind
}
impl Pane{
    fn new(id: usize, ptype: PaneType, title: &str, kind: PaneKind) -> Pane{
        return Pane { 
            _id: id, 
            pane_type: ptype, 
            title: title.to_string(),
            kind: kind
        }
    }

    pub(crate) fn get_kind(&self) -> PaneKind{
        return self.kind
    }

    pub(crate) fn get_type(&self) -> &PaneType{
        return &self.pane_type
    }

    pub(crate) fn reload_with_settings(&mut self, new_settings: FileTreeSettings){
        match &mut self.pane_type {
            PaneType::FileTree { directory, settings } => {
                *settings = new_settings.clone();
                directory.reload(new_settings.show_hidden_elements);
            },
            PaneType::Inspector { .. } => todo!(),
            PaneType::Console { .. } => todo!(),
            PaneType::Empty => todo!(),
        }
    }

    #[deprecated(note="WINIT does not *currently* handle file `dnd` from external sources. Therefore this is not stable.")]
    pub(crate) fn file_dropped(&mut self, path: &PathBuf){
        match &mut self.pane_type{
            PaneType::FileTree { directory, settings } => {
                println!("File dropped in filetree: {}", path.display());
            },
            PaneType::Inspector { variables, new_key, new_value } => {
                println!("File dropped in Inspector: {}", path.display());
            },
            PaneType::Console { messages, input } => {
                println!("File dropped in console: {}", path.display());
            },
            PaneType::Empty => {
                println!("File dropped in empty pane: {}", path.display());
            },
        }
    }
}



pub(crate) struct TreeBehavior {}

impl TreeBehavior{
    fn render_file_tree(&mut self, ui: &mut egui::Ui, directory: &mut UiDirectory, settings: &mut FileTreeSettings) -> UiResponse{
        let mut render_response = None;

        //Render header and drag button
        if let Some(dragged) = self.render_pane_header(ui, "File Tree"){
            render_response = Some(dragged);
        }

        let mut toggled_dirs = Vec::new();
        
        egui::ScrollArea::vertical()
            .max_width(f32::INFINITY)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                //Get TreeNodes from Vec<id>
                let sorted_display_tree = directory.display_tree.clone();
                let visible_items = directory.flat_tree.get_children_from_ids(&sorted_display_tree);
                
                for element in visible_items {                        
                    //Note: if hidden are allowed, or if not hidden
                    let show = settings.show_hidden_elements || (element.file_entry.metadata.file_attributes() & 0x2 == 0);

                    if show{
                        let depth = element.depth;
                        let indent_amount = depth * 20;

                        let element_name = element.file_entry.name.clone();
                        let is_expanded = element.expanded; 
                        
                        ui.horizontal(|ui| {
                            // Indentation based on depth
                            ui.add_space(indent_amount as f32);
                            
                            if element.file_entry.is_dir {
                                let expand_icon = if is_expanded {
                                    "▼"
                                } else {
                                    "▶"
                                };
                                
                                let directory_res = ui.selectable_label(false, format!("📁 {}", element_name));

                                if directory_res.clicked() || ui.button(expand_icon).clicked() {
                                    toggled_dirs.push(element.id);
                                }
                                
                            } else {
                                let _response = ui.selectable_label(false, format!("📄 {}", element_name));
                            }
                        });   
                    }
                    
                }
        });

        if !toggled_dirs.is_empty(){
            toggled_dirs.iter()
                .for_each(|id| {
                    directory.flat_tree.toggle_visibility(id);
            });

            //If even a single dir it toggled, the display tree has to be remade
            let new_sorted = directory.flat_tree.get_visible_items()
                .iter()
                .map(|entry| entry.id)
                .collect();

            directory.display_tree = new_sorted;
        }
        
        //If tile was dragged, return response, else return None.
        if render_response.is_some_and(|status| !matches!(status, egui_tiles::UiResponse::None)){
            return render_response.unwrap()
        }
        return UiResponse::None
    }

    fn render_inspector(&mut self, ui: &mut egui::Ui, variables: &mut HashMap<String, String>, new_key: &mut String, new_value: &mut String) ->UiResponse{

        ui.heading("Variables");
        ui.separator();
        
        // Add new variable section
        ui.horizontal(|ui| {
            ui.label("Key:");
            ui.text_edit_singleline(new_key);
        });
        
        ui.horizontal(|ui| {
            ui.label("Value:");
            ui.text_edit_singleline(new_value);
        });
        
        if ui.button("Add Variable").clicked() && !new_key.is_empty() {
            variables.insert(new_key.clone(), new_value.clone());
            new_key.clear();
            new_value.clear();
        }
        
        ui.separator();
        
        // Display existing variables
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut to_remove = None;
            
            for (key, value) in variables.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", key));
                    ui.text_edit_singleline(value);
                    if ui.button("🗑").clicked() {
                        to_remove = Some(key.clone());
                    }
                });
            }
            
            if let Some(key) = to_remove {
                variables.remove(&key);
            }
        });
        UiResponse::None
    }

    fn render_console(&mut self, ui: &mut egui::Ui, messages: &mut Vec<String>, input: &mut String) -> UiResponse{

        ui.heading("Console");
        ui.separator();
        
        // Messages display
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 60.0)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                for message in messages.iter() {
                    ui.label(message);
                }
            });
        
        ui.separator();
        
        // Input area
        ui.horizontal(|ui| {
            ui.label(">");
            let response = ui.text_edit_singleline(input);
            
            let mut should_process = false;
            
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                should_process = true;
            }
            
            if ui.button("Send").clicked() {
                should_process = true;
            }
            
            if should_process && !input.is_empty() {
                let command = input.clone();
                messages.push(format!("> {}", command));
                
                // Process command
                match command.as_str() {
                    "clear" => messages.clear(),
                    "help" => messages.push("Available commands: clear, help, hello".to_string()),
                    "hello" => messages.push("Hello there!".to_string()),
                    _ => messages.push(format!("Unknown command: {}", command)),
                }
                
                input.clear();
            }
        });
        UiResponse::None
    }

    fn render_empty(&mut self, ui: &mut egui::Ui) -> UiResponse{
        ui.centered_and_justified(|ui| {
            ui.heading("Empty Pane");
            ui.label("This pane is ready for your content!");
        });
        UiResponse::None
    }

    ///Renders the header as well as the drag button
    fn render_pane_header(&mut self, ui: &mut egui::Ui, title: &str) -> Option<egui_tiles::UiResponse>{
        let mut uiresponse = None;

        ui.horizontal(|ui| {
            ui.columns(3, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    ui.heading(title);
                });
                cols[2].vertical_centered_justified(|ui| {
                    let drag_button = egui::Button::new("📌")
                        // .corner_radius(egui::CornerRadius::default())
                        // .fill(Color32::from_rgb(211, 211, 255))
                        .sense(egui::Sense::drag());

                    if ui
                        .add(drag_button)
                        .drag_started()
                    {
                        uiresponse = Some(egui_tiles::UiResponse::DragStarted)
                    } else {
                        //egui_tiles::UiResponse::None
                        uiresponse = None
                    }
                })
            });
            
        });

        ui.separator();

        match uiresponse{
            Some(res) => return Some(res),
            None => return None,
        }
    }
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        format!("Pane {}", pane.title).into()
    }

    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: egui_tiles::TileId, pane: &mut Pane) -> egui_tiles::UiResponse {
        match &mut pane.pane_type{
            PaneType::FileTree { directory, settings} => {
                self.render_file_tree(ui, directory, settings)
            },
            PaneType::Inspector { variables, new_key, new_value } => {
                self.render_inspector(ui, variables, new_key, new_value)
            },
            PaneType::Console { messages, input } => {
                self.render_console(ui, messages, input)
            },
            PaneType::Empty => {
                self.render_empty(ui)
            },
        }
    }
}

///Entry point for initializing and retrieving the Layout tree.
pub(crate) fn create_tree(settings: EditorSettings) -> Result<egui_tiles::Tree<Pane>, EditorIoError> {
    let mut tiles = Tiles::default();

    let mut tree_builder = TreeBuilder::init(None)?;
    let _ = tree_builder.build()?;

    let tree = tree_builder.get_tree();
    
    let visible = tree.get_visible_items()
        .iter()
        .map(|entry| entry.id)
        .collect();
    
    let directory: UiDirectory = {
        UiDirectory { 
            flat_tree: tree,
            display_tree: visible,
        }
    };

    let file_tree = tiles.insert_pane(Pane::new(
        0,
        PaneType::FileTree { directory: directory, settings: settings.into() },
        "File Tree",
        PaneKind::FileTree
    ));
    
    let variables = tiles.insert_pane(Pane::new(
        1,
        PaneType::Inspector {
            variables: {
                let mut vars = HashMap::new();
                vars.insert("debug".to_string(), "true".to_string());
                vars.insert("max_iterations".to_string(), "100".to_string());
                vars
            },
            new_key: String::new(),
            new_value: String::new(),
        },
        "Variables",
        PaneKind::Inspector
    ));
    
    let console = tiles.insert_pane(Pane::new(
        2,
        PaneType::Console {
            messages: vec![
                "Console initialized".to_string(),
                "Type 'help' for available commands".to_string(),
            ],
            input: String::new(),
        },
        "Console",
        PaneKind::Inspector
    ));
    
    let empty = tiles.insert_pane(Pane::new(
        3,
        PaneType::Empty,
        "Main Editor",
        PaneKind::Empty
    ));

    // Bottom section: just the console
    let bottom_section = console;
    
    // Middle section: file tree on left, empty in middle, variables on right
    let middle_section = tiles.insert_horizontal_tile(vec![file_tree, empty, variables]);
    
    // Root: middle section on top, console on bottom
    let root = tiles.insert_vertical_tile(vec![middle_section, bottom_section]);
    
    Ok(Tree::new("main_layout", root, tiles))
}