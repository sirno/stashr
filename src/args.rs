use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File names of files to stash
    pub files: Option<Vec<String>>,

    /// Stash to be used
    #[arg(short, long, default_value = "default")]
    pub stash: String,
}
