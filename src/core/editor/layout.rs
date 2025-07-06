use std::{collections::{BTreeMap, HashMap}, path::Path};

use egui_tiles::{Tiles, Tree, UiResponse};

use crate::utils::{error::EditorIoError, io::{self, DirectoryEntry}};


#[derive(Debug, Clone)]
pub(crate) struct UiDirectory{
    //Element name, Option shows whether element is directory.
    elements: BTreeMap<String, Option<Vec<String>>>,
    //Shows whether or not a `folder` is expanded.
    is_expanded: HashMap<String, bool>
}

#[derive(Debug, Clone)]
pub(crate) enum PaneType{
    FileTree{
        // directory_elements: Vec<(String, Vec<String>)>,
        // expanded_folders: HashMap<String, bool>,
        // files: Vec<String>
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
    // fn render_file_tree(&mut self, ui: &mut egui::Ui, folders: &mut Vec<(String, Vec<String>)>, expanded_folders: &mut HashMap<String, bool>, files: &mut Vec<String>) -> UiResponse{
    //     ui.heading("File Tree");
    //     ui.separator();
        
    //     egui::ScrollArea::vertical().show(ui, |ui| {
    //         for (folder_name, folder_files) in folders {
    //             let is_expanded = expanded_folders.get(folder_name).copied().unwrap_or(false);
                
    //             ui.horizontal(|ui| {
    //                 let expand_icon = if is_expanded { "▼" } else { "▶" };
    //                 if ui.button(expand_icon).clicked() {
    //                     expanded_folders.insert(folder_name.to_string(), !is_expanded);
    //                 }
    //                 ui.label(format!("📁 {}", folder_name));
    //             });
                
    //             if is_expanded {
    //                 ui.indent(format!("indent_{}", folder_name), |ui| {
    //                     for file in folder_files {
    //                         ui.horizontal(|ui| {
    //                             ui.add_space(10.0);
                                
    //                             if ui.selectable_label(false, format!("📄 {}", file)).clicked() {
    //                                 // Only add if not already present to prevent duplicates
    //                                 let selection = format!("Selected: {}/{}", folder_name, file);
                                    
    //                                 if !files.contains(&selection) {
    //                                     files.push(selection);
    //                                 }
    //                             }
    //                         });
    //                     }
    //                 });
    //             }
    //         }
    //     });
    //     UiResponse::None
    // }
    fn render_file_tree(&mut self, ui: &mut egui::Ui, directory: &mut UiDirectory) -> UiResponse{
        ui.heading("File Tree");
        ui.separator();

        let elements = directory.elements.keys().collect::<Vec<&String>>();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for element_name in elements{
                let children = directory.elements.get(element_name).unwrap();

                if children.is_none(){
                    ui.indent(format!("indent_{}", element_name), |ui| {
                        ui.add_space(10.0);
                        
                        let _ = ui.selectable_label(false, format!("📄 {}", element_name));
                    });
                }
                else{
                    let is_expanded = *directory.is_expanded.get(element_name).unwrap_or(&false);
                    let expand_icon = if is_expanded { "▼" } else { "▶" };

                    ui.horizontal(|ui| {    
                        ui.label(format!("📁 {}", element_name));

                        if ui.button(expand_icon).clicked() {
                            directory.is_expanded.entry(element_name.to_string())
                                .and_modify(|expanded| *expanded = !*expanded)
                                .or_insert(false);
                        }
                    });

                    if is_expanded {
                        ui.indent(format!("indent_{}", element_name), |ui| {
                            for file in children.as_ref().unwrap() {
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    
                                    let _ = ui.selectable_label(false, format!("📄 {}", file));
                                });
                            }
                        });
                    }
                }
            }
        });

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
            // PaneType::FileTree { directory_elements, expanded_folders, files } => {
            //     self.render_file_tree(ui, directory_elements, expanded_folders, files)
            // },
            PaneType::FileTree { directory} => {
                //self.render_file_tree(ui, directory_elements, expanded_folders, files)
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
    
    // let tree_folders = Vec::from([
    //     ("src".to_string(), vec!["main.rs".to_string(), "lib.rs".to_string(), "utils.rs".to_string()]),
    //     ("assets".to_string(), vec!["icon.png".to_string(), "config.toml".to_string()]),
    //     ("docs".to_string(), vec!["README.md".to_string(), "CHANGELOG.md".to_string()]),
    // ]);
    let current_directory = io::read_directory(Path::new("."));

    if current_directory.is_err(){
        return Err(current_directory.unwrap_err().into())
    }

    let directory: UiDirectory = {
        let mut elements: BTreeMap<String, Option<Vec<String>>> = BTreeMap::new();
        let mut is_expanded: HashMap<String, bool> = HashMap::new();

        for entry in current_directory.unwrap(){
            match entry{
                DirectoryEntry::File(file) => {
                    let file_name = file.file_name().into_string();
                    
                    elements.insert(file_name.unwrap_or("Unknown".to_string()), None);
                },
                DirectoryEntry::Directory(dir) => {
                    let dir_name = dir.father.file_name().into_string();
                    let mut dir_children = dir.children;
                    
                    dir_children.sort();

                    let sort_children = dir_children.iter()
                        .map(|entry| {
                            entry.get_file_name()
                        })
                        .collect::<Vec<String>>();
                    
                    elements.insert(dir_name.clone().unwrap_or("Unknown".to_string()), Some(sort_children)); 
                    is_expanded.insert(dir_name.unwrap_or("Unknown".to_string()), false);
                }
            }
        }

        UiDirectory { 
            elements: elements, 
            is_expanded: is_expanded 
        }
    };

    let file_tree = tiles.insert_pane(Pane::new(
        0,
        // PaneType::FileTree {
        //     directory_elements: tree_folders,
        //     expanded_folders: HashMap::new(),
        //     files: Vec::new(),
        //},
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