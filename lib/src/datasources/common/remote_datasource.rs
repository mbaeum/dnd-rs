use graphql_client::{Error as GraphQLError, GraphQLQuery, Response};
use log;
use reqwest;
use reqwest::{Error as ReqwestError, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum APIError {
    GraphQL(GraphQLError),
    Reqwest(ReqwestError),
    ResponseHttp(StatusCode, String),
}

#[derive(Debug)]
pub struct GraphQLAPI {
    api_url: String,
}

impl GraphQLAPI {
    pub fn new(api_url: String) -> Self {
        log::debug!("Routing API calls to '{}'", api_url);

        Self { api_url }
    }
    pub async fn get_response_data<
        Ser: Serialize,
        De: for<'de> Deserialize<'de>,
        Query: GraphQLQuery<Variables = Ser, ResponseData = De>,
    >(
        &self,
        variables: Ser,
    ) -> Result<Query::ResponseData, APIError> {
        let client = reqwest::Client::new();
        let request_body = <Query>::build_query(variables);

        log::debug!("Query Body: {}", request_body.query);

        let request_builder = client.post(&self.api_url).json(&request_body);
        log::debug!("Built Request: {request_builder:?}");

        let response = match request_builder.send().await {
            Ok(res) => res,
            Err(err) => {
                log::error!("{err}");
                return Err(APIError::Reqwest(err));
            }
        };

        log::debug!("Got response: {:?}", response);

        let status = response.status();
        let canonical_reason = status.canonical_reason().unwrap_or("Unknown").to_string();
        if status != StatusCode::OK {
            return Err(APIError::ResponseHttp(status, canonical_reason));
        }
        let response_body: Response<Query::ResponseData> = match response.json().await {
            Ok(response_body) => response_body,
            Err(err) => return Err(APIError::Reqwest(err)),
            // Err(err) => return Err(APIError::CustomError(format!("Error in response: {}", err))),
        };
        let data: Query::ResponseData = match response_body.data {
            Some(data) => data,
            None => {
                return Err(APIError::GraphQL(
                    // in case of multiple errors reported, pick first
                    match response_body.errors {
                        Some(errors) => match errors.first() {
                            Some(error) => error.clone(),
                            None => GraphQLError {
                                message: "No error found in response".to_owned(),
                                extensions: None,
                                locations: None,
                                path: None,
                            },
                        },
                        None => GraphQLError {
                            message: "No errors reported".to_owned(),
                            extensions: None,
                            locations: None,
                            path: None,
                        },
                    },
                ));
            }
        };
        Ok(data)
    }
}
