use crate::core::usecase::{dice::DiceImplementation, spell::SpellImplementation};
use crate::core::usecase::{dice::DiceInterface, spell::SpellInterface};
use crate::datasources::{
    common::remote_datasource::GraphQLAPI, spells::spells_datasource::SpellsGraphQLDataSource,
};

use crate::config::settings::Settings;

use log::{debug, error, info};

pub struct SettingsHandler {
    settings: Settings,
}

impl SettingsHandler {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
    pub fn setup_spell_usecase(&self) -> impl SpellInterface {
        match self
            .settings
            .spell_settings
            .spell_datasource
            .remote_type
            .clone()
        {
            x if x == "graphql" => {
                info!("Fetching GraphQL Datasource");
                Self::setup_graphql_spell_usecase(&self.settings)
            }
            _ => {
                error!("Unknown data source requested!");
                panic!("Unknown type")
            }
        }
    }

    pub fn setup_dice_usecase(&self) -> impl DiceInterface {
        DiceImplementation::new()
    }

    fn setup_graphql_spell_usecase(
        settings: &Settings,
    ) -> SpellImplementation<SpellsGraphQLDataSource> {
        debug!("Setting up SpellsGraphQLDataSource");
        let api = GraphQLAPI::new(settings.spell_settings.spell_api.url.clone());
        let datasource = SpellsGraphQLDataSource::new(api);
        SpellImplementation::<SpellsGraphQLDataSource>::new(datasource)
    }
}
