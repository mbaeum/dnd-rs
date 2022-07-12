use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct SpellModel {
    name: Option<String>,
    level: f64,
    desc: Option<Vec<Option<String>>>,
    url: Option<String>,
    index: Option<String>,
}

impl SpellModel {
    pub fn new(
        name: Option<String>,
        level: f64,
        desc: Option<Vec<Option<String>>>,
        url: Option<String>,
        index: Option<String>,
    ) -> Self {
        Self {
            name,
            level,
            desc,
            url,
            index,
        }
    }
}

impl Display for SpellModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "SpellModel {{ name: {:#?}, level: {}, url: {:#?} }}",
            match self.name {
                Some(ref name) => name,
                None => match self.index {
                    Some(ref index) => index,
                    None => "None",
                },
            },
            self.level,
            match self.url {
                Some(ref url) => url,
                None => "None",
            },
        )
    }
}
