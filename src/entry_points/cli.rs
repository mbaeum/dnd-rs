use clap::{Args, Parser, Subcommand};

/// CLI for D&D 5e shenanigans
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// Enter Spells API
    RandomSpell(Random),
}

#[derive(Args, Debug)]
/// Get random spell
pub struct Random {
    #[clap(
        short,
        long,
        help = "Level of spell (by default this is maximum level, get exact with -e, minimum with -m)"
    )]
    pub level: Option<f64>,
    #[clap(
        short,
        long,
        value_delimiter = ',',
        help = "Comma-separated list of classes"
    )]
    pub classes: Vec<String>,
    #[clap(short, long, takes_value(false), help = "Get spells for exact <LEVEL>")]
    pub exact_level: bool,
}
