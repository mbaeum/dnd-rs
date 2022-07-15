pub mod core;
pub mod datasources;
pub mod entry_points;

use crate::core::usecase::random_spell::{RandomSpell, RandomSpellInterface};
use crate::datasources::common::remote_datasource::GraphQLAPI;
use crate::datasources::spells::spells_datasource::SpellsGraphQLDataSource;
use crate::entry_points::cli::{Arguments, SubCommand};
use clap::Parser;

fn setup_random_spell_usecase() -> RandomSpell<SpellsGraphQLDataSource> {
    let api_url = "https://www.dnd5eapi.co/graphql".to_string();
    let api = GraphQLAPI::new(api_url);
    let datasource = SpellsGraphQLDataSource::new(api);
    let cache_time = Some(1000);
    RandomSpell::new(datasource, cache_time)
}
fn map_use_case(args: Arguments) {
    match args.cmd {
        SubCommand::RandomSpell(args) => {
            let mut usecase = setup_random_spell_usecase();
            let spell = usecase
                .get_random_spell(args.level, args.classes, args.exact_level)
                .unwrap();
            println!("{}", spell);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    map_use_case(args);
}
