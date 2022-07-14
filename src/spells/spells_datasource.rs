use super::spell_model::{AbilityScoreSkillsModel, SpellModel};
use super::spell_queries::{spells_query, SpellsQuery};
use crate::datasources::remote_datasource::*;
use futures::executor::block_on;

#[derive(Debug)]
pub enum SpellsDataSourceError {
    GraphQLError(APIError),
    NoSpellsFound,
}

pub trait SpellsDataSource {
    fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError>;
}

#[derive(Debug)]
pub struct SpellsGraphQLDataSource {
    api: GraphQLAPI,
}

impl SpellsGraphQLDataSource {
    pub fn new(api: GraphQLAPI) -> Self {
        Self { api }
    }

    fn make_variables(&self) -> spells_query::Variables {
        spells_query::Variables { limit: Some(0) }
    }

    async fn get_all_raw_spells(
        &self,
    ) -> Result<spells_query::ResponseData, SpellsDataSourceError> {
        let variables = self.make_variables();
        let response_data = self
            .api
            .get_response_data::<spells_query::Variables, spells_query::ResponseData, SpellsQuery>(
                variables,
            )
            .await;
        match response_data {
            Ok(data) => Ok(data),
            Err(err) => Err(SpellsDataSourceError::GraphQLError(err)),
        }
    }

    fn spell_query_classes_to_model(
        &self,
        classes: Option<Vec<Option<spells_query::SpellsQuerySpellsClasses>>>,
    ) -> Option<Vec<AbilityScoreSkillsModel>> {
        classes.map(|classes| {
            classes
                .into_iter()
                .flatten()
                .map(|class| AbilityScoreSkillsModel::new(class.name, class.index))
                .collect()
        })
    }

    fn spell_query_to_model(
        &self,
        spells: Vec<spells_query::SpellsQuerySpells>,
    ) -> Vec<SpellModel> {
        spells
            .into_iter()
            .map(|spell| -> SpellModel {
                SpellModel::new(
                    spell.name,
                    spell.level,
                    spell.desc,
                    spell.url,
                    spell.index,
                    self.spell_query_classes_to_model(spell.classes),
                )
            })
            .collect::<Vec<SpellModel>>()
    }
}

impl SpellsDataSource for SpellsGraphQLDataSource {
    fn get_all_spells(&self) -> Result<Vec<SpellModel>, SpellsDataSourceError> {
        let data = block_on(self.get_all_raw_spells());
        let spells = data?.spells;
        Ok(self.spell_query_to_model(spells))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data_source() -> SpellsGraphQLDataSource {
        let api = GraphQLAPI::new("".to_string());
        SpellsGraphQLDataSource::new(api)
    }

    #[test]
    fn test_make_variables() {
        let data_source = data_source();
        let variables = data_source.make_variables();
        assert_eq!(variables.limit, Some(0));
    }

    #[test]
    fn test_get_from_invalid_url() {
        let data_source = data_source();
        let err = block_on(data_source.get_all_raw_spells());
        assert!(err.is_err());
    }
}
