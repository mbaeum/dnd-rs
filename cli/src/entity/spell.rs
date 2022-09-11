use lib::core::entity::spell::Spell;
use std::fmt::{Display, Error, Formatter};

pub struct CliDisplaySpell<'a>(&'a Spell);

impl CliDisplaySpell<'_> {
    pub fn new(spell: &Spell) -> CliDisplaySpell {
        CliDisplaySpell(spell)
    }
}

impl<'a> Display for CliDisplaySpell<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "-----{}----", "-".repeat(self.0.name.len()))?;
        writeln!(f, "-----{}----", self.0.name)?;
        writeln!(f, "Level:")?;
        writeln!(f, "\t{}", self.0.level)?;
        if !self.0.classes.is_empty() {
            writeln!(f, "Classes:")?;
            writeln!(f, "\t{}", self.0.classes.join(", "))?;
        }
        if !self.0.desc.is_empty() {
            writeln!(f, "Description:")?;
            writeln!(f, "\t{}", self.0.desc.join("\n\t"))?;
        }
        writeln!(f, "-----{}----", "-".repeat(self.0.name.len()))?;
        writeln!(f, "-----{}----", "-".repeat(self.0.name.len()))?;

        Ok(())
    }
}
