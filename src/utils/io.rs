use std::{cmp::Ordering, fs::{self, DirEntry}, os::windows::fs::{FileTypeExt, MetadataExt}, path::Path};

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

                if let Ok(metadata) = entry_unwraped.metadata(){
                    //FIXME: Hotfix to now show hidden files
                    if metadata.file_attributes() & 0x00000002 == 0{

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
                }
            }
        },
        Err(err) => {
            return Err(err.into())
        },
    }

    return Ok(directory_tree)
}


#[derive(Debug)]
pub(crate) enum DirectoryEntry{
    File(DirEntry),
    Directory(DirectoryBlueprint)
}
impl DirectoryEntry{
    pub(crate) fn get_file_name(&self) -> String{
        match self{
            DirectoryEntry::File(dir_entry) => return dir_entry.file_name().into_string().unwrap_or("Unknown".to_string()),
            DirectoryEntry::Directory(directory_blueprint) => directory_blueprint.get_file_name(),
        }
    }
}


#[derive(Debug)]
pub(crate) struct DirectoryBlueprint{
    pub(crate) father: DirEntry,
    pub(crate) children: Vec<DirectoryEntry>
}
impl DirectoryBlueprint{
    pub(crate) fn get_file_name(&self) -> String{
        return self.father.file_name().into_string().unwrap_or("Unknown".to_string())
    }
}