use super::spell_model::SpellModel;
use super::spells_datasource::{SpellsDataSource, SpellsDataSourceError};
use crate::datasources::local_datasource::LocalDatasource;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum SpellRepositoryError {
    DataSourceError(SpellsDataSourceError),
}

pub trait TraitSpellRepository {
    fn get_random_spell(&mut self) -> Result<SpellModel, SpellRepositoryError>;
}

pub struct SpellRepository<T>
where
    T: SpellsDataSource,
{
    datasource: T,
    local_datasource: LocalDatasource<SpellModel>,
}

impl<T> SpellRepository<T>
where
    T: SpellsDataSource,
{
    pub fn new(datasource: T, cache_time: Option<u64>) -> Self {
        let cache_time = cache_time.unwrap_or_else(|| 1000);
        let local_datasource = LocalDatasource::<SpellModel>::new(1, cache_time);
        Self {
            datasource,
            local_datasource,
        }
    }

    fn get_all_spells_and_cache(&mut self) -> Result<Vec<SpellModel>, SpellRepositoryError> {
        //try timed cache before making api call
        match self.local_datasource.get_recent(None) {
            Some(spells) => Ok(spells.to_vec()),
            None => {
                //make api call
                match self.datasource.get_all_spells() {
                    Ok(spells) => {
                        //store in local cache
                        let cache_spells = spells.clone();
                        self.local_datasource.insert(cache_spells, None);
                        Ok(spells)
                    }
                    //on network failure, try permamnent cache
                    Err(e) => match self.local_datasource.get(None) {
                        Some(spells) => Ok(spells),
                        None => Err(SpellRepositoryError::DataSourceError(e)),
                    },
                }
            }
        }
    }
}

impl<T> TraitSpellRepository for SpellRepository<T>
where
    T: SpellsDataSource,
{
    fn get_random_spell(&mut self) -> Result<SpellModel, SpellRepositoryError> {
        let cached_spells = self.get_all_spells_and_cache();
        match cached_spells {
            Ok(spells) => {
                let mut rng = rand::thread_rng();
                let random_spell = spells.choose(&mut rng);
                match random_spell {
                    Some(spell) => Ok(spell.clone()),
                    None => Err(SpellRepositoryError::DataSourceError(
                        SpellsDataSourceError::NoSpellsFound,
                    )),
                }
            }
            Err(e) => Err(e),
        }
    }
}
