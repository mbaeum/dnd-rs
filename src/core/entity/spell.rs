use std::fmt::{Display, Error, Formatter};
pub struct Spell {
    pub name: String,
    pub level: f64,
    pub desc: Vec<String>,
    pub classes: Vec<String>,
}

impl Display for Spell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "-----{}----", "-".repeat(self.name.len()))?;
        writeln!(f, "-----{}----", self.name)?;
        writeln!(f, "Level:")?;
        writeln!(f, "\t{}", self.level)?;
        if !self.classes.is_empty() {
            writeln!(f, "Classes:")?;
            writeln!(f, "\t{}", self.classes.join(", "))?;
        }
        if !self.desc.is_empty() {
            writeln!(f, "Description:")?;
            writeln!(f, "\t{}", self.desc.join("\n\t"))?;
        }
        writeln!(f, "-----{}----", "-".repeat(self.name.len()))?;
        writeln!(f, "-----{}----", "-".repeat(self.name.len()))?;

        Ok(())
    }
}
