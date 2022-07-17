use crate::core::usecase::dice::{DiceError, DiceInterface};
use crate::core::usecase::spell::{SpellError, SpellInterface};
use clap::{Args, Parser, Subcommand};

#[derive(Debug)]
pub enum CliError {
    UnknownSubCommand(String),
    Clap(clap::Error),
    Spell(SpellError),
    Dice(DiceError),
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
    Spell(SpellArgs),
    /// Enter Dice API
    Dice(DiceArgs),
}

#[derive(Args, Clone, Debug)]
/// Get random spell unless name is specified
pub struct SpellArgs {
    #[clap(short, long, takes_value(false), help = "Get random spell")]
    pub random: bool,
    #[clap(short, long, help = "Get spell by name")]
    name: Option<String>,
    #[clap(
        short,
        long,
        help = "Level of spell (by default this is maximum level, get exact with -e)"
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
pub struct DiceArgs {
    #[clap(
        value_delimiter = ' ',
        help = "Space-separated list of dice (e.g. 1d20+2 1d3)"
    )]
    pub dice_sets: Vec<String>,
}

pub struct MainCli<R, D>
where
    R: SpellInterface,
    D: DiceInterface,
{
    random_spell_usecase: R,
    dice_roll_usecase: D,
    args: Arguments,
}

impl<R, D> MainCli<R, D>
where
    R: SpellInterface,
    D: DiceInterface,
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
            SubCommand::Spell(args) => self.handle_spell_cmd(&args),
            SubCommand::Dice(args) => self.handle_dice_cmd(&args),
        }
    }

    pub fn handle_spell_cmd(&mut self, args: &SpellArgs) -> Result<(), CliError> {
        match args.name.clone() {
            Some(name) => {
                let spell = self
                    .random_spell_usecase
                    .get_spell_by_name(name)
                    .map_err(CliError::Spell)?;
                println!("{}", spell);
            }
            None => {
                if args.random {
                    let spell = self
                        .random_spell_usecase
                        .get_random_spell(args.level, args.classes.to_vec(), args.exact_level)
                        .map_err(CliError::Spell)?;
                    println!("{}", spell);
                } else {
                    let spells = self
                        .random_spell_usecase
                        .get_all_spells_with_filters(
                            args.level,
                            args.classes.to_vec(),
                            args.exact_level,
                        )
                        .map_err(CliError::Spell)?;
                    for spell in spells {
                        println!("{}", spell);
                    }
                }
            }
        }

        Ok(())
    }
    pub fn handle_dice_cmd(&mut self, args: &DiceArgs) -> Result<(), CliError> {
        let dice_set = self
            .dice_roll_usecase
            .roll(args.dice_sets.to_vec())
            .unwrap();
        println!("{}", dice_set);
        Ok(())
    }
}
