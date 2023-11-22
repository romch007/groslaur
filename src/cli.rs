use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub enum Command {
    Update,
    Install { packages: Vec<String> },
    Search { term: String },
}
