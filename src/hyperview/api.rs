use anyhow::Result;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_with::{serde_as, DefaultOnError};
use std::fmt;

use super::{auth::get_auth_header, cli::AppConfig};

const BACNET_API_PREFIX: &str = "/api/setting/bacnetIpDefinitions";
const SENSOR_TYPE_ASSET_TYPE: &str = "/api/setting/sensorTypeAssetType";

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
            "id: {}\nasset type: {}\nassociated_assets: {}",
            self.id, self.asset_type, self.associated_assets
        )
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SensorType {
    #[serde(alias = "abbreviatedUnit")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    _abbreviated_unit: String,
    #[serde(alias = "isManuallyCreatable")]
    _is_manually_creatable: bool,
    #[serde(alias = "minimumValidValue")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    _minimum_valid_value: String,
    #[serde(alias = "sensorDescription")]
    sensor_description: String,
    #[serde(alias = "sensorParentType")]
    _sensor_parent_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "unitDescription")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit_description: String,
    #[serde(alias = "unitId")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit_id: String,
}

impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Id: {}\nDiscription: {}\nUnit id: {}\nUnit: {}",
            self.sensor_type_id, self.sensor_description, self.unit_id, self.unit_description
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

pub fn get_sensor_type_asset_type_map(
    config: &AppConfig,
    query: Vec<(String, String)>,
) -> Result<Vec<SensorType>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!("{}{}", config.instance_url, SENSOR_TYPE_ASSET_TYPE);

    // Start http client
    let req = reqwest::blocking::Client::new();

    // Get response
    let resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .query(&query)
        .send()?
        .json::<Vec<SensorType>>()?;

    Ok(resp)
}
