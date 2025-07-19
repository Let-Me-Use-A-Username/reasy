use egui::Ui;

use crate::core::editor::objects::{settings::EditorSettings, ui_tree::PaneKind};


pub(crate) struct EditorMenu{}

impl EditorMenu{
    ///Collects `PaneKinds` of which settings have been altered.
    pub(crate) fn ui(&mut self, ui: &mut Ui, settings: &mut EditorSettings) -> Option<Vec<PaneKind>>{
        let mut ui_changes = vec![];

        egui::menu::bar(ui, |ui| {
            ui.menu_button("Settings", |ui| {
                ui.menu_button("File Tree", |ui| {
                    if ui.checkbox(&mut settings.show_hidden_elements, "Show Hidden items").clicked(){
                        ui_changes.push(PaneKind::FileTree);
                    }
                    if ui.button("Show Hidden items").clicked() {
                        // handle action
                    }
                });
                if ui.button("Item 1").clicked() {
                    // handle action
                }

                if ui.button("Item 2").clicked() {
                    // handle action
                }
            });
        });

        ui_changes.into()
    }
}