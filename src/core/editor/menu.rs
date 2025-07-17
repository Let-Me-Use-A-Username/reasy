use egui::Ui;

use crate::core::editor::objects::editor_settings::EditorSettings;


type PANE_ID = usize;

pub(crate) struct EditorMenu{

}
impl EditorMenu{
    ///NOTE: Return parameter is a `pane id` the change came from.
    pub(crate) fn ui(&mut self, ui: &mut Ui, settings: &mut EditorSettings) -> Option<PANE_ID>{
        let mut changed_settings = None;

        egui::menu::bar(ui, |ui| {
            ui.menu_button("Settings", |ui| {
                ui.menu_button("File Tree", |ui| {
                    //Note: Value is automatically changed. No need to change it manually
                    if ui.checkbox(&mut settings.show_hidden_elements, "Show Hidden items").clicked(){
                        changed_settings = Some(0);
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

        return changed_settings
    }
}