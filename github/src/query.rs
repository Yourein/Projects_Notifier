use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[
    graphql(
        schema_path = "../schema.json",
        query_path = "graphql/authenticate.graphql",
        variables_derives = "Debug",
        response_derives = "Debug"
    )
]
pub struct Authenticate;