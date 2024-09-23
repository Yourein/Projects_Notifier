use std::time::Duration;
use anyhow::anyhow;
use graphql_client::{GraphQLQuery, Response};
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::{model::ProjectTask, query::{
    authenticate,
    get_organization_projects,
    get_project_tasks,
    user_organizations,
    Authenticate,
    GetOrganizationProjects,
    GetProjectTasks,
    ProjectV2Task,
    UserOrganizations
}};

const GRAPHQL_ENDPOINT: &'static str = "https://api.github.com/graphql";
pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(token: &str) -> Client {
        let mut headers = HeaderMap::new();
        let mut access_token = HeaderValue::try_from(format!{"bearer {}", token}).unwrap();
        access_token.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, access_token);

        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("Yourein/ProjectsNotifier")
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client
        }
    }

    fn post_query<Q: GraphQLQuery, ResponseData>(
        &self,
        variables: Q::Variables,
    ) -> anyhow::Result<ResponseData>
    where
        ResponseData: DeserializeOwned + Debug
    {
        let body = Q::build_query(variables);
        let res = self.client.post(GRAPHQL_ENDPOINT)
            .json(&body)
            .send()?;

        // If you want to debug, see raw response data, uncomment below
        // eprintln!{"{:?}", &res.text()}
        // Err(anyhow!{"Return Err anyway!"})

        if res.status().is_success() {
            let json = res.json::<Response<ResponseData>>();
            if json.is_ok() {
                let data = json.unwrap().data;

                if data.is_some() {
                    Ok(data.unwrap())
                } else {
                    Err(
                        anyhow!{"Response does not have data!"}
                    )
                }
            } else {
                Err(
                    anyhow!(json.err().unwrap())
                )
            }
        } else {
            return Err(
                anyhow!{"Status is not 2xx: {}", res.status().as_u16()}
            )
        }
    }

    pub fn get_user_login(&self) -> anyhow::Result<authenticate::ResponseData> {
        let variables = authenticate::Variables {};
        self.post_query::<Authenticate, authenticate::ResponseData>(variables)
    }

    pub fn get_user_organizations(&self) -> anyhow::Result<user_organizations::ResponseData> {
        let variables = user_organizations::Variables {};
        self.post_query::<UserOrganizations, user_organizations::ResponseData>(variables)
    }

    pub fn get_organization_projects(
        &self,
        organization_login: &str,
    ) -> anyhow::Result<get_organization_projects::ResponseData> {
        let variables = get_organization_projects::Variables {
            org_login: organization_login.to_string(),
        };
        self.post_query::<GetOrganizationProjects, get_organization_projects::ResponseData>(variables)
    }

    pub fn get_project_tasks(
        &self,
        organization_login: &str,
        project_serial: i64,
        initial_paging_key: Option<String>,
    ) -> anyhow::Result<Vec<ProjectTask>> {
        let mut tasks: Vec<ProjectV2Task> = vec!();
        let mut next_paging_key: Option<String> = initial_paging_key;

        loop {
            let variables = get_project_tasks::Variables {
                org_login: organization_login.to_string(),
                project_serial: project_serial,
                task_cursor_after: next_paging_key.clone(),
            };

            let res: get_project_tasks::ResponseData = self.post_query::<GetProjectTasks, get_project_tasks::ResponseData>(variables).unwrap();
            
            let project = res
                .organization.unwrap()
                .project_v2.unwrap();

            next_paging_key = project.items.page_info.end_cursor;
            
            if let Some(fetched_tasks) = project.items.nodes {
                tasks.extend(fetched_tasks.into_iter().flatten());
            }

            if !project.items.page_info.has_next_page {
                break;
            }
        }

        let res = tasks.into_iter()
            .map(|it| it.try_into().unwrap())
            .collect::<Vec<ProjectTask>>();

        Ok(res)
    }
}