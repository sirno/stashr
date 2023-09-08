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

fn move_file(origin: &Path, target: &Path) -> Result<(), std::io::Error> {
    match std::fs::rename(origin, target) {
        Ok(_) => Ok(()),
        Err(_) => {
            std::fs::copy(origin, target)?;
            std::fs::remove_file(origin)?;
            Ok(())
        }
    }
}

impl Stash {
    pub fn load(name: String) -> Result<Self, std::io::Error> {
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home).join(".cache/stashr/").join(&name).clone();

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

    pub fn push(&mut self, files: Vec<String>) {
        files.iter().for_each(|f| {
            let file_path = Path::new(f);
            if !file_path.exists() {
                println!("stashr: {}: No such file or directory", file_path.display());
                return;
            }
            let file_name = file_path.file_name().unwrap();
            let path = self.path.join(format!(
                "{}_{}",
                self.latest + 1,
                file_name.to_string_lossy()
            ));
            move_file(file_path, &path).unwrap();
        });
        self.latest += 1;
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
                move_file(&stash_file, Path::new(target)).unwrap();
            }
        }
    }
}
