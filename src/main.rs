pub mod datasources;
pub mod entry_points;
pub mod spells;

use crate::entry_points::dnd_cli::Arguments;
use clap::Parser;

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
}
