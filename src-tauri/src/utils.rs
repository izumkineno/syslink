use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::sync::{Arc, RwLock};
use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::database::{FileSave, SledSet};

const DB_NAME: &str = "db";
pub fn get_sled() -> Result<sled::Db> {
    let db = sled::Config::default()
        .use_compression(true)
        .compression_factor(10)
        .path(DB_NAME)
        .open()?;
    Ok(db)
}

pub fn read_sled() -> Result<Vec<FileSave>> {
    let db: sled::Db = get_sled()?;
    let tree_names = db.tree_names();
    let db_arc = Arc::new(RwLock::new(db));
    let fss = tree_names.par_iter().map(|v| {
        std::str::from_utf8(v).unwrap().to_string()
    }).filter(|v| {
        v != "__sled__default" && v.len() == 6 
    }).map(|v| {
        let mut fs = FileSave::default();
        let db = db_arc.read().unwrap();
        fs.read(&db, v.as_str()).unwrap();
        fs
    }).collect::<Vec<_>>();
    Ok(fss)
}



pub fn random_id(num: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
            .take(num)
            .map(char::from)
            .collect()
}

pub fn random_id_num(len: u64) -> u64 {
    let mut rng = rand::thread_rng();

    let min = 10_u64.pow((len - 1) as u32);
    let max = 10_u64.pow(len as u32) - 1;

    rng.gen_range(min..=max)
}


#[test]
fn test_read_sled() {
    let a = read_sled().unwrap();
    println!("{:?}", a);
}