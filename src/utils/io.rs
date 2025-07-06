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

    fn is_dir(&self) -> bool {
        matches!(self, DirectoryEntry::Directory(_))
    }

    fn classify_char(c: char) -> (u8, char) {
        // Define class rank:
        // 0: lowercase
        // 1: uppercase
        // 2: digit
        // 3: other (special chars)
        let class = match c {
            'a'..='z' => 0,
            'A'..='Z' => 1,
            '0'..='9' => 2,
            _ => 3,
        };
        (class, c)
    }

    fn compare_names(a: &str, b: &str) -> Ordering {
        let mut a_chars = a.chars();
        let mut b_chars = b.chars();

        loop {
            match (a_chars.next(), b_chars.next()) {
                (Some(ac), Some(bc)) => {
                    let ac_key = Self::classify_char(ac);
                    let bc_key = Self::classify_char(bc);
                    let ord = ac_key.cmp(&bc_key);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                (None, Some(_)) => return Ordering::Less, // a is shorter
                (Some(_), None) => return Ordering::Greater, // b is shorter
                (None, None) => return Ordering::Equal,
            }
        }
    }

    pub(crate) fn sort_entries(entries: &mut Vec<DirectoryEntry>){
        entries.sort(); // uses Ord we defined

        for entry in entries {
            if let DirectoryEntry::Directory(blueprint) = entry {
                Self::sort_entries(&mut blueprint.children); // sort recursively
            }
        }
    }
}

impl PartialEq for DirectoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.is_dir() == other.is_dir() && self.get_file_name() == other.get_file_name()
    }
}
impl Eq for DirectoryEntry {}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_dir(), other.is_dir()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => Self::compare_names(&self.get_file_name(), &other.get_file_name()),
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