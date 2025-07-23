mod app;
mod args;
mod ops;
mod stash;

use clap::Parser;

use crate::app::App;
use crate::args::Args;

fn main() {
    let args = Args::parse();

    App::main(args);
}
