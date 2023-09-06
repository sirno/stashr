use itertools::Itertools;
use std::path::{Path, PathBuf};

pub struct Stash {
    name: String,
    path: PathBuf,
    latest: usize,
}

impl Stash {
    pub fn load(name: String) -> Result<Self, std::io::Error> {
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home).join(".cache/stashr/").join(&name).clone();

        std::fs::create_dir_all(&path).unwrap();

        let files = std::fs::read_dir(&path)?;

        let latest: usize = files
            .map(|f| {
                let p = f.unwrap().path();
                p.display()
                    .to_string()
                    .split("_")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .max()
            .unwrap_or_default();

        Ok(Self { name, path, latest })
    }
}
