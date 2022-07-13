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
        let cache_time = cache_time.unwrap_or(1000);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spells::spell_model::SpellModel;

    fn default_spell_model() -> SpellModel {
        SpellModel::new(
            Some("default".to_string()),
            1.3,
            Some(vec![Some("default".to_string())]),
            Some("default".to_string()),
            Some("default-index".to_string()),
        )
    }

    impl SpellsDataSource for LocalDatasource<SpellModel> {
        /// implementing this here to that we can use it as mock
        fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError> {
            Ok(vec![default_spell_model()])
        }
    }

    fn make_repository() -> SpellRepository<LocalDatasource<SpellModel>> {
        let datasource = LocalDatasource::<SpellModel>::new(1, 1000);
        SpellRepository::new(datasource, None)
    }

    #[test]
    fn test_get_all_spells_cache() {
        let mut repository = make_repository();
        let _ = repository.get_all_spells_and_cache();

        assert_eq!(
            repository.local_datasource.get(Some(0_u8)),
            Some(vec![default_spell_model()])
        );

        assert_eq!(
            repository.local_datasource.get_recent(Some(0_u8)),
            Some(&vec![default_spell_model()])
        );
    }

    #[test]
    fn test_get_random_spell() {
        let mut repository = make_repository();
        let random_spell = repository.get_random_spell().unwrap();
        assert_eq!(random_spell, default_spell_model());
    }
}
