use crate::core::usecase::dice_roll::{DiceRollError, DiceRollInterface};
use crate::core::usecase::random_spell::{RandomSpellError, RandomSpellInterface};
use clap::{Args, Parser, Subcommand};

#[derive(Debug)]
pub enum CliError {
    UnknownSubCommand(String),
    Clap(clap::Error),
    RandomSpell(RandomSpellError),
    DiceRoll(DiceRollError),
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
    /// Enter Dice API
    DiceRoll(DiceRollArgs),
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

#[derive(Args, Clone, Debug)]
/// Roll some dice
pub struct DiceRollArgs {
    #[clap(
        short,
        long,
        value_delimiter = ',',
        help = "Comma-separated list of dice (e.g. 1d20+2)"
    )]
    pub dice_sets: Vec<String>,
}

pub struct MainCli<R, D>
where
    R: RandomSpellInterface,
    D: DiceRollInterface,
{
    random_spell_usecase: R,
    dice_roll_usecase: D,
    args: Arguments,
}

impl<R, D> MainCli<R, D>
where
    R: RandomSpellInterface,
    D: DiceRollInterface,
{
    pub fn new(random_spell_usecase: R, dice_roll_usecase: D) -> Self {
        MainCli {
            random_spell_usecase,
            dice_roll_usecase,
            args: Arguments::parse(),
        }
    }
    pub fn run(&mut self) -> Result<(), CliError> {
        match self.args.cmd.clone() {
            SubCommand::RandomSpell(args) => self.handle_random_spell_cmd(&args),
            SubCommand::DiceRoll(args) => self.handle_dice_roll_cmd(&args),
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
    pub fn handle_dice_roll_cmd(&mut self, args: &DiceRollArgs) -> Result<(), CliError> {
        let dice_set = self
            .dice_roll_usecase
            .roll(args.dice_sets.to_vec())
            .unwrap();
        println!("{}", dice_set);
        Ok(())
    }
}
