use anyhow::Result;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::fmt;

use super::{auth::get_auth_header, cli::AppConfig};

const BACNET_API_PREFIX: &str = "/api/setting/bacnetIpDefinitions";

#[derive(Debug, Deserialize)]
pub struct BacnetDefinition {
    id: String,
    #[serde(alias = "assetType")]
    asset_type: String,
    #[serde(alias = "associatedAssets")]
    associated_assets: usize,
}

impl fmt::Display for BacnetDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\nid: {}\nasset type: {}\nassociated_assets: {}\n",
            self.id, self.asset_type, self.associated_assets
        )
    }
}

pub fn get_bacnet_definition_list(config: &AppConfig) -> Result<Vec<BacnetDefinition>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!("{}{}", config.instance_url, BACNET_API_PREFIX);

    // Start http client
    let req = reqwest::blocking::Client::new();

    // Get response
    let resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()?
        .json::<Vec<BacnetDefinition>>()?;

    Ok(resp)
}
