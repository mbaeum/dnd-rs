pub mod cli;
pub mod datasources;
pub mod spells;

use crate::cli::dnd_cli::Arguments;
use clap::Parser;

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
