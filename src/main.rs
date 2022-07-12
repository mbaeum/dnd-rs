mod datasources;
mod spells;
use crate::spells::spells_datasource::SpellsGraphQLDataSource;
use crate::spells::spells_repository::{SpellRepository, TraitSpellRepository};
#[tokio::main]
async fn main() {
    const SPELLS_API_URL: &str = "https://www.dnd5eapi.co/graphql";
    let datasource = SpellsGraphQLDataSource::new(SPELLS_API_URL.to_string());
    let repository = SpellRepository::new(datasource);
    let spell = repository.get_random_spell().unwrap();
    println!("{:?}", spell);
}
