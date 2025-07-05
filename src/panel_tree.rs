use once_cell::sync::OnceCell;
use std::sync::Mutex;


//static TREE: OnceCell<Mutex<Tree<MyTab>>> = OnceCell::new();

enum Pane {
    Settings,
    Text(String),
}

fn tree_ui(ui: &mut egui::Ui, tree: &mut egui_tiles::Tree<Pane>, settings: &mut Settings) {
    let mut behavior = MyBehavior { settings };
    tree.ui(&mut behavior, ui);
}

struct MyBehavior<'a> {
    settings: &'a mut Settings
}

impl<'a> egui_tiles::Behavior<Pane> for MyBehavior<'a> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Settings => "Settings".into(),
            Pane::Text(text) => text.clone().into(),
        }
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::Settings => self.settings.ui(ui),
            Pane::Text(text) => {
                ui.text_edit_singleline(text);
            },
        }

        Default::default()
    }

    // you can override more methods to customize the behavior further
}

struct Settings {
    checked: bool,
}

impl Settings {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.checked, "Checked");
    }
}

