use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct SpellModel {
    pub name: Option<String>,
    pub level: f64,
    pub desc: Option<Vec<Option<String>>>,
    pub url: Option<String>,
    pub index: Option<String>,
    pub classes: Option<Vec<AbilityScoreSkillsModel>>,
}

impl SpellModel {
    pub fn new(
        name: Option<String>,
        level: f64,
        desc: Option<Vec<Option<String>>>,
        url: Option<String>,
        index: Option<String>,
        classes: Option<Vec<AbilityScoreSkillsModel>>,
    ) -> Self {
        Self {
            name,
            level,
            desc,
            url,
            index,
            classes,
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

#[derive(Debug, PartialEq, Clone)]
pub struct AbilityScoreSkillsModel {
    pub name: Option<String>,
    pub index: Option<String>,
}

impl AbilityScoreSkillsModel {
    pub fn new(name: Option<String>, index: Option<String>) -> Self {
        Self { name, index }
    }
}
