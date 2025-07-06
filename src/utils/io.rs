use std::{fs::{self, DirEntry}, os::windows::fs::FileTypeExt, path::Path};

use crate::utils::error::{EditorIoError, ErrorType};


pub(crate) fn read_directory(path: &Path) -> Result<Vec<DirectoryEntry>, EditorIoError>{
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
                let entry_type = entry_unwraped.file_type();

                match entry_type{
                    Ok(ftype) => {
                        //If is file, append
                        if ftype.is_file() | ftype.is_symlink() | ftype.is_symlink_file(){
                            directory_tree.push(DirectoryEntry::File(entry_unwraped));
                        }
                        //If is directory, recursively add dir entries
                        else if ftype.is_dir(){
                            //If success in child directory 
                            if let Ok(child_dir) = read_directory(entry_unwraped.path().as_path()){
                                //Create and push blueprint
                                let dir_blueprint = DirectoryBlueprint { 
                                    father: entry_unwraped,
                                    children: child_dir
                                };
                                directory_tree.push(DirectoryEntry::Directory(dir_blueprint));
                            }
                        }
                    },
                    //Failed getting entry type
                    Err(err) => return Err(err.into()),
                }
            }
        },
        Err(err) => {
            return Err(err.into())
        },
    }

    //TODO: Sort directory tree

    return Ok(directory_tree)
}


pub(crate) enum DirectoryEntry{
    File(DirEntry),
    Directory(DirectoryBlueprint)
}

pub(crate) struct DirectoryBlueprint{
    father: DirEntry,
    children: Vec<DirectoryEntry>
}