use std::{ fmt::Display, fs, os::windows::fs::{symlink_dir, symlink_file, FileTypeExt}, path::Path, sync::{Arc, Mutex}};
use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

use crate::{database::{FileInfo, FileSave, SledSet, SledGet}, utils::{random_id, random_id_num}};


fn link<P: AsRef<Path>, Q: AsRef<Path>>(_type: &SymlinkType, original: &P, link: &Q) -> Result<()> {
    match _type {
        SymlinkType::File => symlink_file(original, link)?,
        SymlinkType::Dir => symlink_dir(original, link)?,
        SymlinkType::Hard => fs::hard_link(original, link)?
    }
    Ok(())
}

// 获取 文件名/文件夹名
fn get_path_last<'a>(path: &'a str) -> Result<&'a str> {
    let name = Path::new(path)
        .file_name()
        .context("获取文件名失败")?
        .to_str()
        .unwrap_or("undefined");
    Ok(name)
}


enum SymlinkType {
    File,
    Dir,
    Hard,
}

impl Display for SymlinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymlinkType::File => write!(f, "文件"),
            SymlinkType::Dir => write!(f, "文件夹"),
            SymlinkType::Hard => write!(f, "硬链接"),
        }
    }
}

enum LinkType {
    File,
    Files,
    Dir,
    Dirs,
    DirAllFiles,
    DirSurface,
    Hard,
    Hards,
    HardsDirAllFiless,
}

impl LinkType {
    // 数字转为枚举
    pub fn from(num: u8) -> LinkType {
        match num {
            0 => LinkType::File,
            1 => LinkType::Files,
            2 => LinkType::Dir,
            3 => LinkType::Dirs,
            4 => LinkType::DirAllFiles,
            5 => LinkType::DirSurface,
            6 => LinkType::Hard,
            7 => LinkType::Hards,
            8 => LinkType::HardsDirAllFiless,
            _ => LinkType::File
        }
    }
}

impl Display for LinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkType::File => write!(f, "单文件"),
            LinkType::Files => write!(f, "多文件"),
            LinkType::Dir => write!(f, "单文件夹"),
            LinkType::Dirs => write!(f, "多文件夹"),
            LinkType::DirAllFiles => write!(f, "所有文件"),
            LinkType::DirSurface => write!(f, "文件夹第一层所有"),
            LinkType::Hard => write!(f, "单文件（硬）"),
            LinkType::Hards => write!(f, "多文件（硬）"),
            LinkType::HardsDirAllFiless => write!(f, "所有文件（硬）"),
        }
    }
}

pub struct Symlink<'a> {
    // 源
    source: Vec<&'a str>,
    // 目标路径
    target_path: &'a str,
    // 连接类型
    _type: LinkType,
    // 自定义文件名（单）
    name: Option<&'a str>,
    // 链接后保存名
    link_name: &'a str,
    // 是否包含软连接
    include_symlinks: bool,
}


impl<'a> Symlink<'_> {
    pub fn new(source: Vec<& 'a str>, target_path: &'a str, _type: u8, link_name: &'a str, name: Option<&'a str>) -> Symlink<'a> {
        Symlink {
            source,
            target_path,
            _type: LinkType::from(_type),
            link_name,
            name,
            include_symlinks: false
        }
    }

    fn link_one(&mut self, link: &dyn Fn(&str, &String)) -> Result<()> {
        // 获取文件名
        let target_name = get_path_last(self.source.get(0).context("未选择文件")?)?;
        // 拼接映射后的路径
        let name = self.name.unwrap_or(&target_name);
        let target = format!("{}\\{}", &self.target_path, name);
        link(&self.source[0], &target);
        Ok(())
    }

    fn link_multi(&mut self, link: &(dyn Fn(&str, &String) + Sync + Send)) -> Result<()> {
        self.source.par_iter().for_each(|s| {
            // 获取文件名
            let name = get_path_last(s).unwrap();
            // 拼接映射后的路径
            let target = format!("{}\\{}", &self.target_path, name);
            link(s, &target)
        });
        Ok(())
    }
}

impl Symlink<'_> {
            // 文件夹下所有文件 不考虑软连接文件夹
            fn link_dir_all_files(&mut self, link: &(dyn Fn(&str, &String)  + Sync + Send)) -> Result<()> {
                WalkDir::new(self.source[0])
                    .into_iter()
                    .par_bridge()
                    .for_each(|entry| {
                        if let std::result::Result::Ok(entry) = entry {
                            let source = entry.path().to_str().unwrap();
                            let child = source.replace(&self.source[0], "");
                            let target = format!("{}\\{}", &self.target_path, child);
                            let use_symlink = self.include_symlinks;
                            if entry.file_type().is_file() || (use_symlink && entry.file_type().is_symlink_file()) {
                                link(source, &target)
                            } else if entry.file_type().is_dir() || (use_symlink && entry.file_type().is_symlink_dir()) {
                                fs::create_dir_all(&target).unwrap_or_else(|e| {
                                    println!("创建文件夹失败： {:?}， 目标路径：{}", e, &target);
                                });
                                // println!("{}", target);
                            }
                        }
                    });
                Ok(())
            }
        
            // 文件夹下第一层所有文件和文件夹 不考虑软连接文件夹
            fn link_dir_surfaces(&mut self, link: &(dyn Fn(&str, &String, bool)  + Sync + Send)) -> Result<()> {
                WalkDir::new(&self.source[0])
                    .max_depth(1)
                    .into_iter()
                    .par_bridge()
                    .for_each(|entry| {
                        if let std::result::Result::Ok(entry) = entry {
                            let p = entry.path().to_str().unwrap();
                            // 获取文件名
                            let name = get_path_last(p).unwrap();
                            let use_symlink = self.include_symlinks;
                            let target = format!("{}\\{}", &self.target_path, name);
                            if entry.file_type().is_file() || (use_symlink && entry.file_type().is_symlink_file()) {
                                link(p, &target, false)
                            } else if entry.file_type().is_dir() || (use_symlink && entry.file_type().is_symlink_dir()) {
                                link(p, &target, true)
                            }
                        }
                    });
                Ok(())
            }
        
}

impl Symlink<'_> {
        // 耦合函数
        pub fn link(&mut self) -> Result<()> {
    
            let fis = Arc::new(Mutex::new(Vec::new()));
            let fis_error = Arc::new(Mutex::new(Vec::new()));
            let link_info_collect = |_type: SymlinkType,s: &str, target: &String| {
                let id = random_id_num(8);
                // println!("id: {}, type: {:?}, file: {:?} -> {:?}", id, _type.to_string(), s, target);
                let fi = FileInfo::new(id, _type.to_string(), s.to_string(), target.clone());
                match link(&_type, &s, &target) {
                    Ok(_) => {
                        fis.lock().unwrap().push(fi);
                    },
                    Err(e) => {
                        println!("建立连接失败： {:?}， 目标路径：{}", e, target);
                        fis_error.lock().unwrap().push(fi);
                    }
                }
    
                // #TODO 连接失败日志
                // #TODO 连接失败选择性保存到连接记录
            };
    
            let res = match self._type {
                LinkType::File => self.link_one(&|a, b| {
                    link_info_collect(SymlinkType::File, a, b);
                }),
                LinkType::Files => self.link_multi(&|a, b| {
                    link_info_collect(SymlinkType::File, a, b);
                }),
                LinkType::Dir => self.link_one(&|a, b| {
                    link_info_collect(SymlinkType::Dir, a, b);
                }),
                LinkType::Dirs => self.link_multi(&|a, b| {
                    link_info_collect(SymlinkType::Dir, a, b);
                }),
                LinkType::DirAllFiles => self.link_dir_all_files(&|a, b| {
                    link_info_collect(SymlinkType::File, a, b);
                }),
                LinkType::DirSurface => self.link_dir_surfaces(&|a, b, is_dir| {
                    let t = if is_dir { SymlinkType::Dir } else { SymlinkType::File };
                    link_info_collect(t, a, b);
                }),
                LinkType::Hard => self.link_one(&|a, b| {
                    link_info_collect(SymlinkType::Hard, a, b);
                }),
                LinkType::Hards => self.link_multi(&|a, b| {
                    link_info_collect(SymlinkType::Hard, a, b);
                }),
                LinkType::HardsDirAllFiless => self.link_dir_all_files(&|a, b| {
                    link_info_collect(SymlinkType::Hard, a, b);
                }),
            };
    
            let id = random_id(6);
            let name = self.link_name;
            let source = Path::new(self.source.get(0).context("未选择文件")?).parent().unwrap().to_str().unwrap();
            let target = self.target_path;
            let type_ = self._type.to_string();
            let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
            let mut ds = FileSave::new(id, name, source, target, type_.to_string(), time);
            let files = fis.lock().unwrap().clone();
            ds.set_files(files);
    
            // dbg!(&ds);
    
            ds.save()?;
    
            res
        }
    
}

