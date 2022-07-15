pub mod core;
pub mod datasources;
pub mod entry_points;

use crate::core::usecase::dice_roll::DiceRoll;
use crate::core::usecase::random_spell::RandomSpell;
use crate::datasources::common::remote_datasource::GraphQLAPI;
use crate::datasources::spells::spells_datasource::SpellsGraphQLDataSource;
use crate::entry_points::cli::MainCli;

fn setup_random_spell_usecase() -> RandomSpell<SpellsGraphQLDataSource> {
    let api_url = "https://www.dnd5eapi.co/graphql".to_string();
    let api = GraphQLAPI::new(api_url);
    let datasource = SpellsGraphQLDataSource::new(api);
    let cache_time = Some(1000);
    RandomSpell::<SpellsGraphQLDataSource>::new(datasource, cache_time)
}

fn setup_dice_roll_usecase() -> DiceRoll {
    DiceRoll::new()
}

#[tokio::main]
async fn main() {
    let mut cli = MainCli::new(setup_random_spell_usecase(), setup_dice_roll_usecase());
    cli.run().unwrap();
}
