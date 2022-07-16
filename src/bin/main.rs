use random_spells_cli::core::{usecase::dice_roll::DiceRoll, usecase::random_spell::RandomSpell};
use random_spells_cli::datasources::{
    common::remote_datasource::GraphQLAPI, spells::spells_datasource::SpellsGraphQLDataSource,
};
use random_spells_cli::entry_points::cli::MainCli;

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
