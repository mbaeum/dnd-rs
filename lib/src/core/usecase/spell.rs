use crate::core::entity::spell::Spell;
use crate::datasources::common::remote_datasource::APIError;
use crate::datasources::queries::spells_query::spells_query::SpellsQuerySpells;
use rand::prelude::SliceRandom;

#[derive(Debug)]
pub enum SpellsDataSourceError {
    GraphQLError(APIError),
    NoSpellsFound,
}

pub trait SpellsDataSourceInterface {
    fn get_all_spells(&self) -> Result<Vec<SpellsQuerySpells>, SpellsDataSourceError>;
}

#[derive(Debug)]
pub enum LocalSpellsDataSourceError {
    CacheEmpty,
    RecentCacheEmpty,
}

#[derive(Debug)]
pub enum SpellError {
    NoSpellsFound,
    DataSourceError(SpellsDataSourceError),
    LocalDataSourceError(LocalSpellsDataSourceError),
}

pub trait SpellInterface {
    fn get_random_spell(
        &mut self,
        level: Option<f64>,
        classes: Vec<String>,
        exact_level: bool,
    ) -> Result<Spell, SpellError>;

    fn get_spell_by_name(&mut self, name: String) -> Result<Spell, SpellError>;

    fn get_all_spells_with_filters(
        &mut self,
        level: Option<f64>,
        classes: Vec<String>,
        exact_level: bool,
    ) -> Result<Vec<Spell>, SpellError>;
}

pub struct SpellImplementation<T>
where
    T: SpellsDataSourceInterface,
{
    datasource: T,
}

impl<T> SpellImplementation<T>
where
    T: SpellsDataSourceInterface,
{
    pub fn new(datasource: T) -> Self {
        SpellImplementation { datasource }
    }

    fn get_spells_from_datasource(&self) -> Result<Vec<SpellsQuerySpells>, SpellError> {
        match self.datasource.get_all_spells() {
            Ok(spells) => Ok(spells),
            Err(err) => Err(SpellError::DataSourceError(err)),
        }
    }

    fn get_all_spells(&mut self) -> Result<Vec<SpellsQuerySpells>, SpellError> {
        match self.get_spells_from_datasource() {
            Ok(spells) => Ok(spells),
            Err(err) => Err(err),
        }
    }

    fn filter_spell_for_classes(&self, spell: &SpellsQuerySpells, classes: &[String]) -> bool {
        if classes.is_empty() {
            return true;
        }
        spell.classes.iter().flatten().any(|spell_class| {
            classes.iter().any(|filter_class| -> bool {
                match spell_class {
                    Some(spell_class) => {
                        spell_class.name == Some(filter_class.to_string())
                            || spell_class.index == Some(filter_class.to_string())
                    }
                    None => false,
                }
            })
        })
    }

    fn filter_spells(
        &mut self,
        spells: Vec<SpellsQuerySpells>,
        level: Option<f64>,
        classes: Vec<String>,
        exact_level: bool,
    ) -> Result<Vec<SpellsQuerySpells>, SpellError> {
        let mut filtered_spells = match spells
            .into_iter()
            .filter(|spell| self.filter_spell_for_classes(spell, &classes))
            .collect::<Vec<SpellsQuerySpells>>()
        {
            f if f.is_empty() => return Err(SpellError::NoSpellsFound),
            f => f,
        };
        match level {
            Some(level) => {
                if exact_level {
                    filtered_spells = filtered_spells
                        .into_iter()
                        .filter(|spell| spell.level == level)
                        .collect::<Vec<SpellsQuerySpells>>();
                } else {
                    filtered_spells = filtered_spells
                        .into_iter()
                        .filter(|spell| spell.level <= level)
                        .collect::<Vec<SpellsQuerySpells>>();
                }
                Ok(filtered_spells)
            }
            None => Ok(filtered_spells),
        }
    }

    fn get_random_spell(
        &mut self,
        spells: Vec<SpellsQuerySpells>,
    ) -> Result<SpellsQuerySpells, SpellError> {
        match spells.choose(&mut rand::thread_rng()) {
            Some(spell) => Ok(spell.clone()),
            None => Err(SpellError::NoSpellsFound),
        }
    }

    fn spell_from_spells_query_spells(&self, spell: &SpellsQuerySpells) -> Spell {
        let name = match spell.name.clone() {
            Some(name) => name,
            None => match spell.index.clone() {
                Some(index) => index,
                None => "".to_string(),
            },
        };
        let desc = match spell.desc.clone() {
            Some(desc) => desc
                .into_iter()
                .map(|desc| desc.unwrap_or_default())
                .collect::<Vec<String>>(),
            None => vec![],
        };
        let classes = match spell.classes.clone() {
            Some(classes) => classes
                .into_iter()
                .map(|class| match class {
                    Some(class) => class
                        .name
                        .unwrap_or_else(|| class.index.unwrap_or_default()),
                    None => "".to_string(),
                })
                .collect::<Vec<String>>(),
            None => vec![],
        };

        Spell {
            name,
            level: spell.level,
            desc,
            classes,
        }
    }
}

impl<T> SpellInterface for SpellImplementation<T>
where
    T: SpellsDataSourceInterface,
{
    fn get_random_spell(
        &mut self,
        level: Option<f64>,
        classes: Vec<String>,
        exact_level: bool,
    ) -> Result<Spell, SpellError> {
        let spells = self.get_all_spells()?;
        let filtered_spells = self.filter_spells(spells, level, classes, exact_level)?;
        match self.get_random_spell(filtered_spells) {
            Ok(spell) => Ok(self.spell_from_spells_query_spells(&spell)),
            Err(err) => Err(err),
        }
    }

    fn get_spell_by_name(&mut self, name: String) -> Result<Spell, SpellError> {
        let name = name.to_lowercase().trim().to_string();
        let spells = self.get_all_spells()?;
        let filtered_spells = spells
            .into_iter()
            .filter(|spell| {
                *spell
                    .name
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .to_lowercase()
                    .trim()
                    == name
                    || *spell
                        .index
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .to_lowercase()
                        .trim()
                        == name.clone()
            })
            .collect::<Vec<SpellsQuerySpells>>();
        match filtered_spells {
            f if f.is_empty() => Err(SpellError::NoSpellsFound),
            _ => Ok(self.spell_from_spells_query_spells(&filtered_spells[0])),
        }
    }

    fn get_all_spells_with_filters(
        &mut self,
        level: Option<f64>,
        classes: Vec<String>,
        exact_level: bool,
    ) -> Result<Vec<Spell>, SpellError> {
        let spells = self.get_all_spells()?;
        let filtered_spells = self.filter_spells(spells, level, classes, exact_level)?;
        match filtered_spells {
            f if f.is_empty() => Err(SpellError::NoSpellsFound),
            _ => Ok(filtered_spells
                .into_iter()
                .map(|spell| self.spell_from_spells_query_spells(&spell))
                .collect::<Vec<Spell>>()),
        }
    }
}
