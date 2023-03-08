use crate::AppConfig;
use anyhow::Result;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, ClientId, ClientSecret, Scope,
    TokenResponse, TokenUrl,
};

pub fn get_auth_header(config: &AppConfig) -> Result<String> {
    // Create client
    let client = BasicClient::new(
        ClientId::new(config.client_id.clone()),
        Some(ClientSecret::new(config.client_secret.clone())),
        AuthUrl::new(config.auth_url.clone())?,
        Some(TokenUrl::new(config.token_url.clone())?),
    );

    // fetch token
    let token_result = client
        .exchange_client_credentials()
        .add_scope(Scope::new(config.scope.clone()))
        .request(http_client)?;

    Ok(format!("Bearer {}", token_result.access_token().secret()))
}
