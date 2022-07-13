mod datasources;
mod spells;
use crate::spells::spells_datasource::SpellsGraphQLDataSource;
use crate::spells::spells_repository::{SpellRepository, TraitSpellRepository};
use std::thread::sleep;
use std::time::Duration;
#[tokio::main]
async fn main() {
    const SPELLS_API_URL: &str = "https://www.dnd5eapi.co/graphql";
    let datasource = SpellsGraphQLDataSource::new(SPELLS_API_URL.to_string());
    let mut repository = SpellRepository::new(datasource, None);
    let spell = repository.get_random_spell().unwrap();
    println!("spell1 {}", spell);
    let spell = repository.get_random_spell().unwrap();
    println!("spell2 {}", spell);
    let cache_expire = Duration::from_millis(1000);
    sleep(cache_expire);
    let spell = repository.get_random_spell().unwrap();
    println!("spell3 {}", spell);
}
