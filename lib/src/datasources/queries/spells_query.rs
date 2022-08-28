use self::spells_query::SpellsQuerySpells;
use self::spells_query::SpellsQuerySpellsClasses;

use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery, PartialEq)]
#[graphql(
    query_path = "../queries/spells/spells_query.graphql",
    schema_path = "../queries/spells/schema.graphql",
    response_derives = "Debug"
)]
pub struct SpellsQuery;

impl Clone for SpellsQuerySpellsClasses {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            name: self.name.clone(),
        }
    }
}
impl Clone for SpellsQuerySpells {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            level: self.level,
            desc: self.desc.clone(),
            url: self.url.clone(),
            index: self.index.clone(),
            classes: self.classes.clone(),
        }
    }
}
