use crate::ops::{TransferOp, move_file};
use std::path::{Path, PathBuf};

pub struct Stash {
    name: String,
    path: PathBuf,
    latest: usize,
}

trait DirEntryExt {
    fn file_name_string(&self) -> String;
}

impl DirEntryExt for std::fs::DirEntry {
    fn file_name_string(&self) -> String {
        self.path()
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

impl Stash {
    pub fn load(name: String) -> Result<Self, std::io::Error> {
        let home = std::env::var("HOME").unwrap();
        let cache_dir = std::env::var("STASHR_CACHE_DIR").unwrap_or(".cache/stashr".to_string());
        let path = Path::new(&home).join(cache_dir).join(&name).clone();

        std::fs::create_dir_all(&path).unwrap();

        let files = std::fs::read_dir(&path)?;

        let latest: usize = files
            .map(|f| {
                let p = f
                    .unwrap()
                    .path()
                    .file_name()
                    .map(|f| f.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let n = p.split("_").next().unwrap();
                n.parse::<usize>().unwrap()
            })
            .max()
            .unwrap_or_default();

        Ok(Self { name, path, latest })
    }

    pub fn is_empty(&self) -> bool {
        self.latest == 0
    }

    pub fn push(&mut self, files: Vec<String>, op: TransferOp) {
        self.latest += 1;
        files.iter().for_each(|f| {
            let file_path = Path::new(f);
            if !file_path.exists() {
                println!("stashr: {}: No such file or directory", file_path.display());
                return;
            }
            let file_name = file_path.file_name().unwrap();
            let path = self
                .path
                .join(format!("{}_{}", self.latest, file_name.to_string_lossy()));
            match op(file_path, &path) {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "stashr: {}: Cannot stash file or directory",
                        file_path.display()
                    );
                    // rollback latest
                    self.pop();
                }
            }
        });
    }

    pub fn pop(&mut self) {
        if self.latest == 0 {
            println!("stashr: \"{}\": Stash is empty", self.name);
            return;
        }

        let files = std::fs::read_dir(&self.path).unwrap();

        for file in files {
            let p = match file.as_ref().map(|dir_entry| dir_entry.file_name_string()) {
                Ok(p) => p,
                Err(_) => continue,
            };

            match p.split_once("_") {
                Some((n, target)) if n.parse::<usize>().unwrap_or_default() == self.latest => {
                    let stash_file = file.unwrap().path();
                    let target_path = Path::new(target);
                    match move_file(&stash_file, target_path) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("stashr: {target}: Cannot unstash file or directory");
                            continue;
                        }
                    }
                }
                _ => continue,
            }
        }
        self.latest -= 1;
    }

    pub fn content(&self) -> Vec<(usize, String)> {
        let files = std::fs::read_dir(&self.path).unwrap();
        let mut list = Vec::new();
        for file in files {
            let p = match file.as_ref().map(|dir_entry| dir_entry.file_name_string()) {
                Ok(p) => p,
                Err(_) => continue,
            };
            if let Some((n, target)) = p.split_once("_") {
                list.push((n.parse::<usize>().unwrap_or_default(), target.to_string()));
            }
        }
        list.sort_by_key(|s| s.0);
        list
    }
}

impl std::fmt::Display for Stash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let list = self.content();
        writeln!(f, "{}", self.name)?;
        for (n, target) in list {
            writeln!(f, "  {}: {}", n, target)?;
        }
        Ok(())
    }
}
