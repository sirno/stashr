use crate::args::Args;
use crate::stash::Stash;

pub struct App {}

impl App {
    pub fn main(args: Args) {
        println!("{:?}", args.files);
        let stash = Stash::load(args.stash.clone()).unwrap();
    }
}
