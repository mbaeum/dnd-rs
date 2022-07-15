use crate::core::usecase::random_spell::{SpellsDataSourceError, SpellsDataSourceInterface};
use crate::datasources::common::remote_datasource::GraphQLAPI;
use crate::datasources::queries::spells_query::spells_query::{
    ResponseData, SpellsQuerySpells, Variables,
};
use crate::datasources::queries::spells_query::SpellsQuery;
// use crate::datasources::queries::spells_query;
use futures::executor::block_on;

pub struct SpellsGraphQLDataSource {
    api: GraphQLAPI,
}

impl SpellsGraphQLDataSource {
    pub fn new(api: GraphQLAPI) -> Self {
        Self { api }
    }

    fn make_variables(&self) -> Variables {
        Variables { limit: Some(0) }
    }

    async fn get_all_raw_spells(&self) -> Result<ResponseData, SpellsDataSourceError> {
        let variables = self.make_variables();
        let response_data = self
            .api
            .get_response_data::<Variables, ResponseData, SpellsQuery>(variables)
            .await;
        match response_data {
            Ok(data) => Ok(data),
            Err(err) => Err(SpellsDataSourceError::GraphQLError(err)),
        }
    }
}

impl SpellsDataSourceInterface for SpellsGraphQLDataSource {
    fn get_all_spells(&self) -> Result<Vec<SpellsQuerySpells>, SpellsDataSourceError> {
        let data = block_on(self.get_all_raw_spells());
        let spells = data?.spells;
        Ok(spells)
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
