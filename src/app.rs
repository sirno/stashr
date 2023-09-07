use crate::args::Args;
use crate::stash::Stash;

pub struct App {}

impl App {
    pub fn main(args: Args) {
        let mut stash = Stash::load(args.stash.clone()).unwrap();
        match args.files {
            Some(files) => stash.push(files),
            None => stash.pop(),
        }
    }
}
