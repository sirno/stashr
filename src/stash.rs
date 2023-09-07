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
            let path = self.path.join(format!("{}_{}", self.latest + 1, f));
            std::fs::rename(&f, &path).unwrap();
        });
        self.latest += 1;
    }

    pub fn pop(&mut self) {
        if self.latest == 0 {
            println!("Stash ({}) is empty", self.name);
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
                std::fs::rename(&stash_file, &target).unwrap();
            }
        }
    }
}
