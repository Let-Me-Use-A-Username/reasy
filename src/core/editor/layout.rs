use std::{collections::HashMap};

use egui_tiles::{Tiles, Tree, UiResponse};

use crate::{core::editor::objects::flat_tree::{FlatTree, TreeBuilder}, utils::{error::EditorIoError}};


#[derive(Debug, Clone)]
pub(crate) struct UiDirectory{
    elements: FlatTree,
}

#[derive(Debug, Clone)]
pub(crate) enum PaneType{
    FileTree{
        directory: UiDirectory
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

pub(crate) struct Pane {
    _id: usize, 
    pane_type: PaneType,
    title: String,
}
impl Pane{
    fn new(id: usize, ptype: PaneType, title: &str) -> Pane{
        return Pane { 
            _id: id, 
            pane_type: ptype, 
            title: title.to_string() 
        }
    }
}



pub(crate) struct TreeBehavior {}

impl TreeBehavior{
    fn render_file_tree(&mut self, ui: &mut egui::Ui, directory: &mut UiDirectory) -> UiResponse{
        let mut toggled_dirs = Vec::new();

        ui.heading("File Tree");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            let visible_items = directory.elements.get_visible_items();
            
            for element in visible_items {
                let element_name = element.file_entry.name.clone();
                let is_expanded = element.expanded; 
                let depth = element.depth;
                
                let indent_amount = depth * 20;
                
                ui.horizontal(|ui| {
                    // Indentation based on depth
                    ui.add_space(indent_amount as f32);
                    
                    if element.file_entry.is_dir {
                        let expand_icon = if is_expanded {
                            "▼"
                        } else {
                            "▶"
                        };
                        
                        ui.label(format!("📁 {}", element_name));
                        if ui.button(expand_icon).clicked() {
                            toggled_dirs.push(element.id);
                        }
                    } else {
                        let _response = ui.selectable_label(false, format!("📄 {}", element_name));
                    }
                });
                
            }
        });

        if !toggled_dirs.is_empty(){
            toggled_dirs.iter()
                .for_each(|id| {
                    directory.elements.toggle_visibility(id);
            });
        }
        
        UiResponse::None
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
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        format!("Pane {}", pane.title).into()
    }

    fn pane_ui(&mut self, ui: &mut egui::Ui, _tile_id: egui_tiles::TileId, pane: &mut Pane) -> egui_tiles::UiResponse {
        match &mut pane.pane_type{
            PaneType::FileTree { directory} => {
                self.render_file_tree(ui, directory)
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
pub(crate) fn create_tree() -> Result<egui_tiles::Tree<Pane>, EditorIoError> {
    let mut tiles = Tiles::default();

    let mut tree_builder = TreeBuilder::init()?;
    let _ = tree_builder.build()?;

    let directory: UiDirectory = {
        UiDirectory { 
            elements: tree_builder.get_tree()
        }
    };

    let file_tree = tiles.insert_pane(Pane::new(
        0,
        PaneType::FileTree { directory: directory },
        "File Tree",
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
    ));
    
    let empty = tiles.insert_pane(Pane::new(
        3,
        PaneType::Empty,
        "Main Editor",
    ));

    // Bottom section: just the console
    let bottom_section = console;
    
    // Middle section: file tree on left, empty in middle, variables on right
    let middle_section = tiles.insert_horizontal_tile(vec![file_tree, empty, variables]);
    
    // Root: middle section on top, console on bottom
    let root = tiles.insert_vertical_tile(vec![middle_section, bottom_section]);
    
    Ok(Tree::new("main_layout", root, tiles))
}