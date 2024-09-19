use std::time::Duration;
use anyhow::anyhow;
use graphql_client::{GraphQLQuery, Response};
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::query::{
    authenticate, Authenticate,
    user_organizations, UserOrganizations,
};

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
}