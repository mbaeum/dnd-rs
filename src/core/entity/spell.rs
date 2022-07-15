use std::fmt::{Display, Error, Formatter};
pub struct Spell {
    pub name: String,
    pub level: f64,
    pub desc: Vec<String>,
    pub classes: Vec<String>,
}

impl Display for Spell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "-----{}----", "-".repeat(self.name.len()));
        write!(f, "-----{}----", self.name);
        write!(f, "Level:");
        write!(f, "\t{}", self.level);
        if self.classes.not_empty() {
            write!(f, "Classes:");
            write!(f, "\t{}", self.classes.join(", "));
        }
        if self.desc.not_empty() {
            write!(f, "Description:");
            for desc in self.desc {
                write!(f, "\t{}", desc);
            }
        }

        Ok(())
    }
}
