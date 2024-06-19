
use std::path::Path;
use anyhow::{Ok, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::utils::{get_sled, random_id};


pub trait SledGet {
    fn get_tree_name(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_source(&self) -> &str;
    fn get_target(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_time(&self) -> &str;
    fn get_files(&self) -> &Vec<FileInfo>;
    fn get_files_id(&self) -> &str;

    fn save(&self) -> Result<()> {
        let db = get_sled()?;
        let tree = db.open_tree(self.get_tree_name())?;

        tree.insert("name", self.get_name())?;
        tree.insert("source", self.get_source())?;
        tree.insert("target", self.get_target())?;
        tree.insert("type", self.get_type())?;
        tree.insert("time", self.get_time())?;

        let file_id = random_id(16);
        tree.insert("file_id", file_id.as_bytes())?;
        let tree_files = db.open_tree(file_id)?;

        self.get_files().par_iter().for_each(|f| {
            let f_str = serde_json::to_string(&f).unwrap();
            tree_files.insert(f.id.to_string().as_bytes(), f_str.as_bytes()).unwrap(); 
        });

        Ok(())
    }
}

pub trait SledSet {
    fn set_id(&mut self, name: String);
    fn set_name(&mut self, name: String);
    fn set_source(&mut self, source: String);
    fn set_target(&mut self, target: String);
    fn set_type(&mut self, type_: String);
    fn set_time(&mut self, time: String);
    fn set_files(&mut self, files: Vec<FileInfo>);
    fn set_files_id(&mut self, id: String);

    fn read(&mut self, db: &sled::Db, tree_name: &str) -> Result<()> {
        let tree = db.open_tree(tree_name).unwrap();
        
        let name = tree.get("name").unwrap().unwrap();
        let source = tree.get("source").unwrap().unwrap();
        let target = tree.get("target").unwrap().unwrap();
        let type_ = tree.get("type").unwrap().unwrap();
        let time = tree.get("time").unwrap().unwrap();
        let files = tree.get("file_id").unwrap().unwrap();

        let name = std::str::from_utf8(&name).unwrap();
        let source = std::str::from_utf8(&source).unwrap();
        let target = std::str::from_utf8(&target).unwrap();
        let type_ = std::str::from_utf8(&type_).unwrap();
        let time = std::str::from_utf8(&time).unwrap();
        let file_id = std::str::from_utf8(&files).unwrap();

        self.set_id(tree_name.to_owned());
        self.set_name(name.to_owned());
        self.set_source(source.to_owned());
        self.set_target(target.to_owned());
        self.set_type(type_.to_owned());
        self.set_time(time.to_owned());
        self.set_files_id(file_id.to_owned());

        Ok(())
    }

    fn read_files(&mut self, db: &sled::Db, file_id: &str) {
        let tree_files = db.open_tree(file_id).unwrap();
        let files = tree_files.iter().map(|v| {
            let value = v.unwrap().1;
            let value = std::str::from_utf8(&value).unwrap();
            let file = serde_json::from_str::<FileInfo>(value).unwrap();
            file
        }).collect::<Vec<_>>();
        self.set_files(files);
    }

}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    id: u64,
    type_: String,
    source: String,
    target: String,
}

impl FileInfo {
    pub fn new(id: u64, type_: impl AsRef<str>, source: impl AsRef<str>, target: impl AsRef<str>) -> FileInfo {
        FileInfo {
            id,
            type_: type_.as_ref().to_string(),
            source: source.as_ref().to_string(),
            target: target.as_ref().to_string(),
        }
    }

    pub fn remove_link(&self) -> Result<()> {
        let f = Path::new(&self.target);
        if f.exists() {
            if f.is_file() {
                std::fs::remove_file(&self.target)?;
            } else {
                std::fs::remove_dir(&self.target)?;
            }
        }
        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FileSave {
    id: String,
    name: String,
    source: String,
    target: String,
    type_: String,
    time: String,
    files_id: String,
    files: Vec<FileInfo>,
}

impl FileSave {
    pub fn new(id: impl AsRef<str>, name: impl AsRef<str>, source: impl AsRef<str>, target: impl AsRef<str>, type_: impl AsRef<str>, time: impl AsRef<str>) -> FileSave {
        FileSave {
            id: id.as_ref().to_string(),
            name: name.as_ref().to_string(),
            source: source.as_ref().to_string(),
            target: target.as_ref().to_string(),
            type_: type_.as_ref().to_string(),
            time: time.as_ref().to_string(),
            files_id: String::new(),
            files: Vec::new(),
        }
    }
}

impl SledGet for FileSave {    
    fn get_tree_name(&self) -> &str { &self.id }
    
    fn get_name(&self) -> &str { &self.name }
    
    fn get_source(&self) -> &str { &self.source }
    
    fn get_target(&self) -> &str { &self.target }
    
    fn get_type(&self) -> &str { &self.type_ }
    
    fn get_time(&self) -> &str { &self.time }
    
    fn get_files(&self) -> &Vec<FileInfo> { &self.files }

    fn get_files_id(&self) -> &str { &self.files_id }
}

impl SledSet for FileSave {
    fn set_id(&mut self, id: String) { self.id = id; }

    fn set_name(&mut self, name: String) { self.name = name; }

    fn set_source(&mut self, source: String) { self.source = source; }

    fn set_target(&mut self, target: String) { self.target = target; }

    fn set_type(&mut self, type_: String) { self.type_ = type_; }

    fn set_time(&mut self, time: String) { self.time = time; }

    fn set_files(&mut self, files: Vec<FileInfo>) { self.files = files; }

    fn set_files_id(&mut self, id: String) { self.files_id = id; }
}


