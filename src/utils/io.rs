use std::{fs::{self, DirEntry}, os::windows::fs::{MetadataExt}, path::{Path, PathBuf}};

use crate::utils::error::{EditorIoError, ErrorType};

///Reads and returns a single directory. Does not recurse.
pub(crate) fn read_directory(path: &Path, show_hidden: bool) -> Result<Vec<FileEntry>, EditorIoError>{
    if !path.is_dir(){
        return Err(EditorIoError::new("Path not a directory", ErrorType::NotADirectory))
    }

    let mut directory_tree = vec![];

    match fs::read_dir(path){
        Ok(directory) => {
            for entry in directory{
                //Failed reading directory element
                if entry.is_err(){
                    return Err(entry.unwrap_err().into())
                }
                let entry_unwraped = entry.unwrap();

                if let Ok(metadata) = entry_unwraped.metadata(){
                    let should_show = show_hidden || (metadata.file_attributes() & 0x00000002 == 0);
                    
                    if should_show{
                        let entry_converted: FileEntry = entry_unwraped.into();
                        directory_tree.push(entry_converted.clone());
                    }
                }
            }
        },
        Err(err) => {
            return Err(err.into())
        },
    }

    return Ok(directory_tree)
}



#[derive(Debug, Clone)]
pub(crate) struct FileEntry {
    pub(crate) parent: String,
    pub(crate) name: String,
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) is_file: bool,
    pub(crate) is_symlink: bool,
    pub(crate) size: Option<u64>,
    pub(crate) modified: Option<std::time::SystemTime>,
}

impl Into<FileEntry> for DirEntry {
    fn into(self) -> FileEntry {
        let metadata = self.metadata().unwrap();
        let file_type = metadata.file_type();
        let size = metadata.len();
        let full_path = self.path();

        let parent_path = {
            let mut components = full_path.components();
            let _ = components.next_back();

            if let Some(last) = components.next_back(){
                last.as_os_str().to_string_lossy().to_string()
            }
            else{
                ".".to_string()
            }
        };
        let modified = metadata.modified().ok();

        FileEntry {
            parent: parent_path,
            name: self.file_name().to_string_lossy().to_string(),
            path: full_path,
            is_dir: file_type.is_dir(),
            is_file: file_type.is_file(),
            is_symlink: file_type.is_symlink(),
            size: Some(size),
            modified,
        }
    }
}