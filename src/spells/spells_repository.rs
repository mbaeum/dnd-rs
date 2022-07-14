use super::spell_model::SpellModel;
use super::spells_datasource::{SpellsDataSource, SpellsDataSourceError};
use crate::datasources::local_datasource::LocalDatasource;
use rand::seq::SliceRandom;

pub struct SpellRepositoryFilters {
    pub min_level: Option<f64>,
    pub max_level: Option<f64>,
    pub classes: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum SpellRepositoryError {
    DataSourceError(SpellsDataSourceError),
    FilterError(String),
}

pub trait TraitSpellRepository {
    fn get_random_spell(
        &mut self,
        filters: &SpellRepositoryFilters,
    ) -> Result<SpellModel, SpellRepositoryError>;
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
        let local_datasource = LocalDatasource::<SpellModel>::new(2, cache_time);
        Self {
            datasource,
            local_datasource,
        }
    }

    fn get_all_spells_and_cache(&mut self) -> Result<Vec<SpellModel>, SpellRepositoryError> {
        //try timed cache before making api call
        match self.local_datasource.get_recent(0_u8) {
            Some(spells) => Ok(spells.to_vec()),
            None => {
                //make api call
                match self.datasource.get_all_spells() {
                    Ok(spells) => {
                        //store in local cache
                        let cache_spells = spells.clone();
                        self.local_datasource.insert(cache_spells, 0_u8);
                        Ok(spells)
                    }
                    //on network failure, try permamnent cache
                    Err(e) => match self.local_datasource.get(0_u8) {
                        Some(spells) => Ok(spells),
                        None => Err(SpellRepositoryError::DataSourceError(e)),
                    },
                }
            }
        }
    }

    fn filter_spells(
        &mut self,
        spells: Vec<SpellModel>,
        filters: &SpellRepositoryFilters,
    ) -> Result<Vec<SpellModel>, SpellRepositoryError> {
        match spells
            .into_iter()
            .filter(|spell| {
                if let Some(level) = filters.max_level {
                    spell.level <= level
                } else {
                    true
                }
            })
            .filter(|spell| {
                if let Some(level) = filters.min_level {
                    spell.level >= level
                } else {
                    true
                }
            })
            .filter(|spell| {
                if let Some(classes) = &filters.classes {
                    self.filter_spell_for_class(spell, classes)
                } else {
                    true
                }
            })
            .collect::<Vec<SpellModel>>()
        {
            f if f.is_empty() => Err(SpellRepositoryError::FilterError(
                "No spells found".to_string(),
            )),
            f => Ok(f),
        }
    }

    fn filter_spell_for_class(&self, spell: &SpellModel, classes: &[String]) -> bool {
        spell.classes.iter().flatten().any(|spell_class| {
            classes.iter().any(|filter_class| -> bool {
                spell_class.name == Some(filter_class.to_string())
                    || spell_class.index == Some(filter_class.to_string())
            })
        })
    }
}

impl<T> TraitSpellRepository for SpellRepository<T>
where
    T: SpellsDataSource,
{
    fn get_random_spell(
        &mut self,
        filters: &SpellRepositoryFilters,
    ) -> Result<SpellModel, SpellRepositoryError> {
        let cached_spells = self.get_all_spells_and_cache();
        match cached_spells {
            Ok(spells) => {
                let mut rng = rand::thread_rng();
                match self.filter_spells(spells, filters) {
                    Ok(filtered_spells) => match filtered_spells.choose(&mut rng) {
                        Some(spell) => Ok(spell.clone()),
                        None => Err(SpellRepositoryError::DataSourceError(
                            SpellsDataSourceError::NoSpellsFound,
                        )),
                    },
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spells::spell_model::{AbilityScoreSkillsModel, SpellModel};

    fn dummy_ability_score(name: &str) -> AbilityScoreSkillsModel {
        AbilityScoreSkillsModel::new(Some(name.to_string()), Some(name.to_string()))
    }

    fn dummy_spell_model() -> Vec<SpellModel> {
        vec![SpellModel::new(
            Some("default".to_string()),
            1.3,
            Some(vec![Some("default".to_string())]),
            Some("default".to_string()),
            Some("default-index".to_string()),
            None,
        )]
    }

    fn dummy_multi_spell_model() -> Vec<SpellModel> {
        vec![
            SpellModel::new(
                Some("default".to_string()),
                1.0,
                Some(vec![Some("default".to_string())]),
                Some("default".to_string()),
                Some("default-index".to_string()),
                Some(vec![dummy_ability_score("test_class1")]),
            ),
            SpellModel::new(
                Some("default".to_string()),
                2.0,
                Some(vec![Some("default".to_string())]),
                Some("default".to_string()),
                Some("default-index".to_string()),
                Some(vec![dummy_ability_score("test_class2")]),
            ),
            SpellModel::new(
                Some("default".to_string()),
                3.0,
                Some(vec![Some("default".to_string())]),
                Some("default".to_string()),
                Some("default-index".to_string()),
                Some(vec![dummy_ability_score("test_class3")]),
            ),
        ]
    }

    fn dummy_repo_filters() -> SpellRepositoryFilters {
        SpellRepositoryFilters {
            min_level: None,
            max_level: None,
            classes: None,
        }
    }

    struct SingeSpellDataSourceMock;

    impl SpellsDataSource for SingeSpellDataSourceMock {
        /// implementing this here to that we can use it as mock
        fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError> {
            Ok(dummy_spell_model())
        }
    }

    fn make_single_spell_repository() -> SpellRepository<SingeSpellDataSourceMock> {
        let datasource = SingeSpellDataSourceMock;
        SpellRepository::new(datasource, Some(100))
    }

    struct MultiSpellDataSourceMock;

    impl SpellsDataSource for MultiSpellDataSourceMock {
        /// implementing this here to that we can use it as mock
        fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError> {
            Ok(dummy_multi_spell_model())
        }
    }

    fn make_multi_spell_repository() -> SpellRepository<MultiSpellDataSourceMock> {
        let datasource = MultiSpellDataSourceMock;
        SpellRepository::new(datasource, Some(100))
    }

    #[test]
    fn test_get_all_spells_cache() {
        let mut repository = make_single_spell_repository();
        let _ = repository.get_all_spells_and_cache();

        assert_eq!(
            repository.local_datasource.get(0_u8),
            Some(dummy_spell_model())
        );

        assert_eq!(
            repository.local_datasource.get_recent(0_u8),
            Some(&dummy_spell_model())
        );
    }

    #[test]
    fn test_get_random_spell() {
        let filters = dummy_repo_filters();
        let mut repository = make_single_spell_repository();
        let random_spell = repository.get_random_spell(&filters).unwrap();
        assert_eq!(random_spell, dummy_spell_model()[0]);
    }

    #[test]
    fn test_min_level_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.min_level = Some(3.0);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters).unwrap();
        assert_eq!(filtered[0], dummy_multi_spell_model()[2]);
    }
    #[test]
    fn test_bad_min_evel_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.min_level = Some(4.0);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters);
        assert!(filtered.is_err());
    }

    #[test]
    fn test_max_level_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.max_level = Some(1.0);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters).unwrap();
        assert_eq!(filtered[0], dummy_multi_spell_model()[0]);
    }
    #[test]
    fn test_bad_max_level_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.max_level = Some(0.0);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters);
        assert!(filtered.is_err());
    }
    #[test]
    fn test_classes_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.classes = Some(vec!["test_class3".to_string()]);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters).unwrap();
        assert_eq!(filtered[0], dummy_multi_spell_model()[2]);
    }
    #[test]
    fn test_multi_classes_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.classes = Some(vec!["test_class2".to_string(), "test_class3".to_string()]);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters).unwrap();
        assert_eq!(filtered, dummy_multi_spell_model()[1..]);
    }
    #[test]
    fn test_all_classes_filter_spells() {
        let filters = dummy_repo_filters();
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters).unwrap();
        assert_eq!(filtered, dummy_multi_spell_model());
    }
    #[test]
    fn test_bad_classes_filter_spells() {
        let mut filters = dummy_repo_filters();
        filters.classes = Some(vec!["test_class4".to_string()]);
        let mut repository = make_multi_spell_repository();

        let spells = dummy_multi_spell_model();
        let filtered = repository.filter_spells(spells, &filters);
        assert!(filtered.is_err());
    }
}
