
///Configuration struct that holds *ALL* information regarding ui editor.
#[derive(Debug, Clone)]
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