use crate::args::Args;
use crate::stash::Stash;

pub struct App {}

impl App {
    pub fn main(args: Args) {
        if args.list {
            App::list();
            return;
        }

        let mut stash = Stash::load(args.stash.clone()).unwrap();

        let stash_op = if args.copy {
            crate::stash::copy_file
        } else {
            crate::stash::move_file
        };

        match args.files {
            Some(files) => stash.push(files, stash_op),
            None => stash.pop(),
        }
    }

    pub fn list() {
        let home = std::env::var("HOME").unwrap();
        let path = std::path::Path::new(&home).join(".cache/stashr/");

        let stashes = std::fs::read_dir(&path).unwrap();
        let mut count = 0;
        for stash_path in stashes {
            let stash_name = stash_path
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            let stash = &Stash::load(stash_name).unwrap();
            if stash.is_empty() {
                continue;
            }
            print!("{}", stash);
            count += 1;
        }

        if count == 0 {
            println!("stashr: Nothing is stashed");
        }
    }
}
