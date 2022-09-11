use lib::core::entity::dice_set::{Dice, DiceSet};

use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct CliDisplayDice<'a>(&'a Dice);

impl<'a> Display for CliDisplayDice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}d{}", self.0.dice_count, self.0.face)?;
        if self.0.modifier.is_some() {
            write!(f, "+{}", self.0.modifier.unwrap())?;
        }
        Ok(())
    }
}

pub struct CliDisplayDiceSet<'a>(&'a DiceSet);

impl CliDisplayDiceSet<'_> {
    pub fn new(dice_set: &DiceSet) -> CliDisplayDiceSet {
        CliDisplayDiceSet(dice_set)
    }
}

impl<'a> Display for CliDisplayDiceSet<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Total: \t\t{}", self.0.result)?;
        for (dice, result) in &self.0.dice_map {
            writeln!(f, "|---{}: \t{}", CliDisplayDice(dice), result)?;
        }
        Ok(())
    }
}
