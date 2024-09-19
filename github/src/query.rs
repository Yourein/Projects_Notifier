use graphql_client::GraphQLQuery;
use crate::scaler::DateTime;

pub type ProjectV2Task = get_project_tasks::GetProjectTasksOrganizationProjectV2ItemsNodes;

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

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../schema.json",
    query_path = "graphql/userorganizations.graphql",
    variables_derives = "Debug",
    response_derives = "Debug",
)]
pub struct UserOrganizations;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../schema.json",
    query_path = "graphql/get_organization_projects.graphql",
    variables_derives = "Debug",
    response_derives = "Debug",
)]
pub struct GetOrganizationProjects;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../schema.json",
    query_path = "graphql/getprojecttask.graphql",
    variables_derives = "Debug",
    response_derives = "Debug, Clone",
)]
pub struct GetProjectTasks;