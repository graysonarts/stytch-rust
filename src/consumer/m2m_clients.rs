// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use crate::consumer::m2m::M2MClient;
use crate::consumer::m2m::M2MClientWithClientSecret;
use crate::consumer::m2m::M2MSearchQuery;
use crate::consumer::m2m::ResultsMetadata;
use crate::consumer::m2m_clients_secrets::Secrets;
use serde::{Deserialize, Serialize};

/// CreateRequest: Request type for `Clients.create`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    /// scopes: An array of scopes assigned to the client.
    pub scopes: std::vec::Vec<String>,
    /// client_id: If provided, the ID of the client to create. If not provided, Stytch will generate this value
    /// for you. The `client_id` must be unique within your project.
    pub client_id: std::option::Option<String>,
    /// client_secret: If provided, the stored secret of the client to create. If not provided, Stytch will
    /// generate this value for you. If provided, the `client_secret` must be at least 8 characters long and
    /// pass entropy requirements.
    pub client_secret: std::option::Option<String>,
    /// client_name: A human-readable name for the client.
    pub client_name: std::option::Option<String>,
    /// client_description: A human-readable description for the client.
    pub client_description: std::option::Option<String>,
    /// trusted_metadata: The `trusted_metadata` field contains an arbitrary JSON object of application-specific
    /// data. See the [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior
    /// details.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
}
/// CreateResponse: Response type for `Clients.create`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_client: The M2M Client created by this API call.
    pub m2m_client: M2MClientWithClientSecret,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// DeleteRequest: Request type for `Clients.delete`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeleteRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
}
/// DeleteResponse: Response type for `Clients.delete`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// client_id: The ID of the client.
    pub client_id: String,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// GetRequest: Request type for `Clients.get`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
}
/// GetResponse: Response type for `Clients.get`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_client: The M2M Client affected by this operation.
    pub m2m_client: M2MClient,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// SearchRequest: Request type for `Clients.search`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchRequest {
    /// cursor: The `cursor` field allows you to paginate through your results. Each result array is limited to
    /// 1000 results. If your query returns more than 1000 results, you will need to paginate the responses
    /// using the `cursor`. If you receive a response that includes a non-null `next_cursor` in the
    /// `results_metadata` object, repeat the search call with the `next_cursor` value set to the `cursor` field
    /// to retrieve the next page of results. Continue to make search calls until the `next_cursor` in the
    /// response is null.
    pub cursor: std::option::Option<String>,
    /// limit: The number of search results to return per page. The default limit is 100. A maximum of 1000
    /// results can be returned by a single search request. If the total size of your result set is greater than
    /// one page size, you must paginate the response. See the `cursor` field.
    pub limit: std::option::Option<u32>,
    /// query: The optional query object contains the operator, i.e. `AND` or `OR`, and the operands that will
    /// filter your results. Only an operator is required. If you include no operands, no filtering will be
    /// applied. If you include no query object, it will return all results with no filtering applied.
    pub query: std::option::Option<M2MSearchQuery>,
}
/// SearchResponse: Response type for `Clients.search`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_clients: An array of M2M Clients that match your search query.
    pub m2m_clients: std::vec::Vec<M2MClient>,
    /// results_metadata: The search `results_metadata` object contains metadata relevant to your specific query
    /// like total and `next_cursor`.
    pub results_metadata: ResultsMetadata,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}
/// UpdateRequest: Request type for `Clients.update`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateRequest {
    /// client_id: The ID of the client.
    pub client_id: String,
    /// client_name: A human-readable name for the client.
    pub client_name: std::option::Option<String>,
    /// client_description: A human-readable description for the client.
    pub client_description: std::option::Option<String>,
    /// status: The status of the client - either `active` or `inactive`.
    pub status: std::option::Option<UpdateRequestStatus>,
    /// scopes: An array of scopes assigned to the client.
    pub scopes: std::option::Option<std::vec::Vec<String>>,
    /// trusted_metadata: The `trusted_metadata` field contains an arbitrary JSON object of application-specific
    /// data. See the [Metadata](https://stytch.com/docs/api/metadata) reference for complete field behavior
    /// details.
    pub trusted_metadata: std::option::Option<serde_json::Value>,
}
/// UpdateResponse: Response type for `Clients.update`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// m2m_client: The M2M Client affected by this operation.
    pub m2m_client: M2MClient,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum UpdateRequestStatus {
    #[serde(rename = "active")]
    #[default]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

pub struct Clients {
    http_client: crate::client::Client,
    pub secrets: Secrets,
}

impl Clients {
    pub fn new(http_client: crate::client::Client) -> Self {
        Self {
            http_client: http_client.clone(),
            secrets: Secrets::new(http_client.clone()),
        }
    }

    pub async fn get(&self, body: GetRequest) -> crate::Result<GetResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/m2m/clients/{client_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::GET,
                path,
                body,
            })
            .await
    }
    pub async fn search(&self, body: SearchRequest) -> crate::Result<SearchResponse> {
        let path = String::from("/v1/m2m/clients/search");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
    pub async fn update(&self, body: UpdateRequest) -> crate::Result<UpdateResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/m2m/clients/{client_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::PUT,
                path,
                body,
            })
            .await
    }
    pub async fn delete(&self, body: DeleteRequest) -> crate::Result<DeleteResponse> {
        let client_id = &body.client_id;
        let path = format!("/v1/m2m/clients/{client_id}");
        self.http_client
            .send(crate::Request {
                method: http::Method::DELETE,
                path,
                body,
            })
            .await
    }
    pub async fn create(&self, body: CreateRequest) -> crate::Result<CreateResponse> {
        let path = String::from("/v1/m2m/clients");
        self.http_client
            .send(crate::Request {
                method: http::Method::POST,
                path,
                body,
            })
            .await
    }
}
