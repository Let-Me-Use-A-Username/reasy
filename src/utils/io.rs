use std::{fs::{self, DirEntry, Metadata}, path::{Path, PathBuf}};

use serde::{de::DeserializeOwned, Serialize};

use crate::utils::error::{EditorIoError, ErrorType};

///Reads and returns a single directory. Does not recurse.
pub(crate) fn read_directory(path: &Path) -> Result<Vec<FileEntry>, EditorIoError>{
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

                let entry_converted: FileEntry = entry_unwraped.into();
                directory_tree.push(entry_converted.clone());
            }
        },
        Err(err) => {
            return Err(err.into())
        },
    }

    return Ok(directory_tree)
}

///Reads a file and deserialized into a concrete struct.
pub(crate) fn read_serialized_data<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, EditorIoError>{
    let content = fs::read_to_string(path.as_ref())?;

    let returned_structure = serde_json::from_str::<T>(&content)
        .map_err(|err| EditorIoError::from(err));

    return returned_structure
}

///Writes a serialized struct into a file.
pub(crate) fn write_serialized_data<T: Serialize, P: AsRef<Path>>(settings: &T, path: P) -> Result<(), EditorIoError>{
    let content = serde_json::to_string_pretty(settings)?;
    
    let data_written = fs::write(path, content)
        .map_err(|err| EditorIoError::from(err));

    return data_written
}





///Structure type that imitates a DirEntry, in order to be bale to perform more operations when reading a directory.
#[allow(dead_code)]
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
    pub(crate) metadata: Metadata
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
            metadata
        }
    }
}