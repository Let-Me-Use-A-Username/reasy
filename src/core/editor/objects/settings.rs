use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{utils::{error::EditorIoError, io}, EDITOR_ROOT_DIR};


///Configuration struct that holds *ALL* information regarding ui editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EditorSettings{
    //FileTree
    pub(crate) show_hidden_elements: bool
}

impl Default for EditorSettings{
    fn default() -> EditorSettings {
        return EditorSettings { 
            show_hidden_elements: false 
        }
    }
}

///Reads and returns if exists, previous saved settings. Else returns default.
pub(crate) fn load_settings() -> Result<EditorSettings, EditorIoError>{
    let path = get_settings_path();

    match path.try_exists(){
        Ok(verified) => {
            //Path exists but isn't verified. Return default settings
            if !verified{
                return Ok(EditorSettings::default())
            }
            //Verified path, read it via IO.
            else{
                return io::read_serialized_data(path)
            }
        },
        Err(err) => return Err(err.into()),
    }
}

///Saves settings into Editor Project path.
pub(crate) fn save_settings(settings: &EditorSettings) -> Result<(), EditorIoError>{
    let path = get_settings_path();

    return io::write_serialized_data(settings, path)
}


///Helper function to point to settings file.
/// Should be placed in same depth as `Cargo.toml`
fn get_settings_path() -> PathBuf{
    let root_dir = EDITOR_ROOT_DIR.get().unwrap().as_path();
    let full_path = root_dir.join("settings.json");

    return full_path
}





///Configuration that are related to UI preferences.
#[derive(Clone, Debug)]
pub(crate) struct FileTreeSettings{
    pub(crate) show_hidden_elements: bool
}
impl From<EditorSettings> for FileTreeSettings{
    fn from(value: EditorSettings) -> Self {
        return FileTreeSettings { 
            show_hidden_elements: value.show_hidden_elements 
        }
    }
}