use graphql_client::GraphQLQuery;
#[derive(GraphQLQuery)]
#[graphql(
    query_path = "queries/spells/spells_query.graphql",
    schema_path = "queries/spells/schema.graphql",
    response_derives = "Debug"
)]
pub struct SpellsQuery;

// #[derive(GraphQLQuery)]
// #[graphql(
//     query_path = "queries/spells/spell_by_index_query.graphql",
//     schema_path = "queries/spells/schema.graphql",
//     response_derives = "Debug"
// )]
// struct SpellByIndexQuery;
