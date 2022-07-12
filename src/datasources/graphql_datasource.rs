use graphql_client::{Error as GraphQLError, GraphQLQuery, Response};
use reqwest;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum GraphQLAPIError {
    APIError(GraphQLError),
    ReqwestError(ReqwestError),
}

#[derive(Debug)]
pub struct GraphQLAPI {
    api_url: String,
}

impl GraphQLAPI {
    pub fn new(api_url: String) -> Self {
        Self { api_url }
    }
    pub async fn get_response_data<
        Ser: Serialize,
        De: for<'de> Deserialize<'de>,
        Query: GraphQLQuery<Variables = Ser, ResponseData = De>,
    >(
        &self,
        variables: Ser,
    ) -> Result<Query::ResponseData, GraphQLAPIError> {
        let client = reqwest::Client::new();
        let request_body = <Query>::build_query(variables);
        let response = match client.post(&self.api_url).json(&request_body).send().await {
            Ok(res) => res,
            Err(err) => return Err(GraphQLAPIError::ReqwestError(ReqwestError::from(err))),
        };
        let response_body: Response<Query::ResponseData> = match response.json().await {
            Ok(response_body) => response_body,
            Err(err) => return Err(GraphQLAPIError::ReqwestError(ReqwestError::from(err))),
        };
        let data: Query::ResponseData = match response_body.data {
            Some(data) => data,
            None => {
                return Err(GraphQLAPIError::APIError(
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
