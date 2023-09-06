mod app;
mod args;
mod stash;

use clap::Parser;

use crate::app::App;
use crate::args::Args;
use crate::stash::Stash;

fn main() {
    let args = Args::parse();

    App::main(args);
}
