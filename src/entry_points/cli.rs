use crate::core::usecase::random_spell::{RandomSpellError, RandomSpellInterface};
use clap::{Args, Parser, Subcommand};

#[derive(Debug)]
pub enum CliError {
    UnknownSubCommand(String),
    Clap(clap::Error),
    RandomSpell(RandomSpellError),
}

/// CLI for D&D 5e shenanigans
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum SubCommand {
    /// Enter Spells API
    RandomSpell(RandomSpellArgs),
}

#[derive(Args, Clone, Debug)]
/// Get random spell
pub struct RandomSpellArgs {
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

pub struct MainCli<R>
where
    R: RandomSpellInterface,
{
    random_spell_usecase: R,
    args: Arguments,
}

impl<R> MainCli<R>
where
    R: RandomSpellInterface,
{
    pub fn new(random_spell_usecase: R) -> Self {
        MainCli {
            random_spell_usecase,
            args: Arguments::parse(),
        }
    }
    pub fn run(&mut self) -> Result<(), CliError> {
        match self.args.cmd.clone() {
            SubCommand::RandomSpell(args) => self.handle_random_spell_cmd(&args),
        }
    }

    pub fn handle_random_spell_cmd(&mut self, args: &RandomSpellArgs) -> Result<(), CliError> {
        let spell = self
            .random_spell_usecase
            .get_random_spell(args.level, args.classes.to_vec(), args.exact_level)
            .unwrap();
        println!("{}", spell);
        Ok(())
    }
}
