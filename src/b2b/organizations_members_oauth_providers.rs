// !!!
// WARNING: This file is autogenerated
// Only modify code within MANUAL() sections
// or your changes may be overwritten later!
// !!!

use serde::{Deserialize, Serialize};


/// GoogleResponse: Response type for `OAuthProviders.google`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GoogleResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// provider_type: Denotes the OAuth identity provider that the user has authenticated with, e.g. Google,
    /// Microsoft, GitHub etc.
    pub provider_type: String,
    /// provider_subject: The unique identifier for the User within a given OAuth provider. Also commonly called
    /// the `sub` or "Subject field" in OAuth protocols.
    pub provider_subject: String,
    /// id_token: The `id_token` returned by the OAuth provider. ID Tokens are JWTs that contain structured
    /// information about a user. The exact content of each ID Token varies from provider to provider. ID Tokens
    /// are returned from OAuth providers that conform to the [OpenID Connect](https://openid.net/foundation/)
    /// specification, which is based on OAuth.
    pub id_token: String,
    /// scopes: The OAuth scopes included for a given provider. See each provider's section above to see which
    /// scopes are included by default and how to add custom scopes.
    pub scopes: std::vec::Vec<String>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// access_token: The `access_token` that you may use to access the User's data in the provider's API.
    pub access_token: std::option::Option<String>,
    /// access_token_expires_in: The number of seconds until the access token expires.
    pub access_token_expires_in: std::option::Option<i32>,
    /// refresh_token: The `refresh_token` that you may use to obtain a new `access_token` for the User within
    /// the provider's API.
    pub refresh_token: std::option::Option<String>,
}
/// MicrosoftResponse: Response type for `OAuthProviders.microsoft`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MicrosoftResponse {
    /// request_id: Globally unique UUID that is returned with every API call. This value is important to log
    /// for debugging purposes; we may ask for this value to help identify a specific API call when helping you
    /// debug an issue.
    pub request_id: String,
    /// provider_type: Denotes the OAuth identity provider that the user has authenticated with, e.g. Google,
    /// Microsoft, GitHub etc.
    pub provider_type: String,
    /// provider_subject: The unique identifier for the User within a given OAuth provider. Also commonly called
    /// the `sub` or "Subject field" in OAuth protocols.
    pub provider_subject: String,
    /// access_token: The `access_token` that you may use to access the User's data in the provider's API.
    pub access_token: String,
    /// access_token_expires_in: The number of seconds until the access token expires.
    pub access_token_expires_in: i32,
    /// id_token: The `id_token` returned by the OAuth provider. ID Tokens are JWTs that contain structured
    /// information about a user. The exact content of each ID Token varies from provider to provider. ID Tokens
    /// are returned from OAuth providers that conform to the [OpenID Connect](https://openid.net/foundation/)
    /// specification, which is based on OAuth.
    pub id_token: String,
    /// scopes: The OAuth scopes included for a given provider. See each provider's section above to see which
    /// scopes are included by default and how to add custom scopes.
    pub scopes: std::vec::Vec<String>,
    /// status_code: The HTTP status code of the response. Stytch follows standard HTTP response status code
    /// patterns, e.g. 2XX values equate to success, 3XX values are redirects, 4XX are client errors, and 5XX
    /// are server errors.
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    /// refresh_token: The `refresh_token` that you may use to obtain a new `access_token` for the User within
    /// the provider's API.
    pub refresh_token: std::option::Option<String>,
}
/// ProviderInformationRequest: Request type for `OAuthProviders.google`, `OAuthProviders.microsoft`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProviderInformationRequest {
    /// organization_id: Globally unique UUID that identifies a specific Organization. The `organization_id` is
    /// critical to perform operations on an Organization, so be sure to preserve this value.
    pub organization_id: String,
    /// member_id: Globally unique UUID that identifies a specific Member. The `member_id` is critical to
    /// perform operations on a Member, so be sure to preserve this value.
    pub member_id: String,
    /// include_refresh_token: Whether to return the refresh token Stytch has stored for the OAuth Provider.
    /// Defaults to false. **Important:** If your application exchanges the refresh token, Stytch may not be
    /// able to automatically refresh access tokens in the future.
    pub include_refresh_token: std::option::Option<bool>,
}




pub struct OAuthProviders {
  http_client: crate::client::Client,
}

impl OAuthProviders {
    pub fn new(http_client: crate::client::Client) -> Self {
      Self {
        http_client: http_client.clone(),
      }
    }

    pub async fn google(&self, body: ProviderInformationRequest) -> crate::Result<GoogleResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/members/{member_id}/oauth_providers/google");
        self.http_client.send(crate::Request{
            method: http::Method::GET,
            path,
            body,
        }).await
    }
    pub async fn microsoft(&self, body: ProviderInformationRequest) -> crate::Result<MicrosoftResponse> {
        let organization_id = &body.organization_id;
        let member_id = &body.member_id;
        let path = format!("/v1/b2b/organizations/{organization_id}/members/{member_id}/oauth_providers/microsoft");
        self.http_client.send(crate::Request{
            method: http::Method::GET,
            path,
            body,
        }).await
    }

}
