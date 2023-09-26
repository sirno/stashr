use std::fs;
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

fn move_file(origin: &Path, target: &Path) -> anyhow::Result<()> {
    if origin.is_dir() {
        fs::rename(origin, target)
            .and_then(|_| Ok(()))
            .or_else(|_| {
                let options = fs_extra::dir::CopyOptions::new().content_only(true);
                fs::create_dir_all(target)?;
                fs_extra::dir::copy(origin, target, &options)?;
                fs_extra::dir::remove(origin)
            })
            .map_err(|e| anyhow::anyhow!(e))
    } else if origin.is_file() {
        fs::rename(origin, target)
            .and_then(|_| Ok(()))
            .or_else(|_| {
                let options = fs_extra::file::CopyOptions::new();
                fs_extra::file::copy(origin, target, &options)?;
                fs_extra::file::remove(origin)
            })
            .map_err(|e| anyhow::anyhow!(e))
    } else {
        let e = fs_extra::error::Error::new(
            fs_extra::error::ErrorKind::Other,
            "Not a directory or file",
        );
        Err(anyhow::anyhow!(e))
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

    pub fn push(&mut self, files: Vec<String>) {
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
            match move_file(&file_path, &path) {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "stashr: {}: Cannot move file or directory",
                        file_path.display()
                    );
                    // rollback latest
                    self.pop();
                    return;
                }
            }
        });
    }

    pub fn pop(&mut self) {
        if self.latest == 0 {
            println!("stashr: {} stash is empty", self.name);
            return;
        }

        let files = std::fs::read_dir(&self.path).unwrap();

        for file in files {
            let p = match file.as_ref().map(|dir_entry| dir_entry.file_name_string()) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let mut splits = p.split("_");
            if splits.next().unwrap().parse::<usize>().unwrap() == self.latest {
                let stash_file = file.unwrap().path();
                let target = splits.remainder().unwrap();
                match move_file(&stash_file, Path::new(target)) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("stashr: {}: Cannot move file or directory", target);
                        continue;
                    }
                }
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
            let mut splits = p.split("_");
            let n = splits.next().unwrap().parse::<usize>().unwrap();
            let target = splits.remainder().unwrap();
            list.push((n, target.to_string()));
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
