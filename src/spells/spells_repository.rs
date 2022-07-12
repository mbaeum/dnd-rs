use super::spell_model::SpellModel;
use super::spells_datasource::{SpellsDataSource, SpellsDataSourceError};

use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum SpellRepositoryError {
    DataSourceError(SpellsDataSourceError),
}

pub trait TraitSpellRepository {
    fn get_random_spell(&self) -> Result<SpellModel, SpellRepositoryError>;
}

#[derive(Debug)]
pub struct SpellRepository<T>
where
    T: SpellsDataSource,
{
    datasource: T,
}

impl<T> SpellRepository<T>
where
    T: SpellsDataSource,
{
    pub fn new(datasource: T) -> Self {
        Self { datasource }
    }
}

impl<T> TraitSpellRepository for SpellRepository<T>
where
    T: SpellsDataSource,
{
    fn get_random_spell(&self) -> Result<SpellModel, SpellRepositoryError> {
        match self.datasource.get_all_spells() {
            Ok(spells) => {
                let spell = spells.choose(&mut rand::thread_rng()).unwrap();
                Ok(spell.clone())
            }
            Err(err) => Err(SpellRepositoryError::DataSourceError(err)),
        }
    }
}
