use std::sync::{Arc, Mutex};
use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_json::{json, Value};

use crate::{database::{FileSave, SledGet, SledSet}, symlink::Symlink, utils::{get_sled, read_sled}};

macro_rules! catch_error_to_string {
    ($func:ident, $( $x:expr ),*) => {
        match $func($( $x ),*).await {
            Ok(value) => Ok(value),
            Err(err) => {
                Err(err.to_string())
            }
        }
    };
}

async fn _link<'a>(source: Vec<&'a str>, target: &'a str, t: u8, lname: &'a str, name: Option<&'a str>) -> Result<()> {
    // dbg!(&source, &target, &t, &name);
    let mut symlink = Symlink::new(source, target, t, lname, name);
    symlink.link()
}
#[tauri::command]
pub async fn link<'a>(source: Vec<& 'a str>, target: &'a str, t: u8, lname: &'a str, name: Option<&'a str>) -> Result<(), String> {
    catch_error_to_string!(_link, source, target, t, lname, name)
}

async fn _read_data() -> Result<Vec<FileSave>> {
    let data = read_sled()?;
    // dbg!(&data);
    Ok(data)
}
#[tauri::command]
pub async fn read_sled_from_db() -> Result<Vec<FileSave>, String> {
    catch_error_to_string!(_read_data,)
}

async fn _read_data_files(id: &str, all: bool) -> Result<Value> {
    let mut fs = FileSave::default();
    let db = get_sled()?;
    fs.read_files(&db, id);
    // dbg!(&data);
    let data = if all {
        json!(fs)
    } else {
        json!(fs.get_files())
    };
    Ok(data)
}
#[tauri::command]
pub async fn read_sled_files_from_db(id: &str, all: bool) -> Result<Value, String> {
    catch_error_to_string!(_read_data_files, id, all)
}

async fn _remove_data(fss: Vec<Value>) -> Result<()> {
    // dbg!(&fss);
    let db = Arc::new(Mutex::new(get_sled()?));
    fss.par_iter().for_each(|v| {
        let mut fs = serde_json::from_value::<FileSave>(v.clone()).unwrap();
        let id = { fs.get_tree_name().to_string() };
        let fid = { fs.get_files_id().to_string() };
        
        fs.read_files(&db.lock().unwrap(), &fid);
        fs.get_files().iter().for_each(|v| {
            v.remove_link().unwrap();
        });

        db.lock().unwrap().drop_tree(fid).unwrap();
        db.lock().unwrap().drop_tree(id).unwrap();
    });
    Ok(())
}
#[tauri::command]
pub async fn remove_sled_from_db(fss: Vec<Value>) -> Result<(), String> {
    catch_error_to_string!(_remove_data, fss)
}