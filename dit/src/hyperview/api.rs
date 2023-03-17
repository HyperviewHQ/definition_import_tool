use anyhow::Result;
use log::{error, info};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DefaultOnError};
use std::fmt;
use uuid::Uuid;

use super::{auth::get_auth_header, cli::AppConfig};

const BACNET_API_PREFIX: &str = "/api/setting/bacnetIpDefinitions";
const SENSOR_TYPE_ASSET_TYPE: &str = "/api/setting/sensorTypeAssetType";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetDefinition {
    id: String,
    name: String,
    #[serde(alias = "assetType")]
    asset_type: String,
    #[serde(alias = "associatedAssets")]
    associated_assets: usize,
}

impl fmt::Display for BacnetDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nasset type: {}\nassociated_assets: {}",
            self.id, self.name, self.asset_type, self.associated_assets
        )
    }
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetIpNumericSensor {
    id: String,
    name: String,
    multiplier: f64,
    #[serde(alias = "objectInstance")]
    object_instance: usize,
    #[serde(alias = "objectType")]
    object_type: String,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit: String,
    #[serde(alias = "unitId")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit_id: String,
}

impl fmt::Display for BacnetIpNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nmultiplier: {}\nobject_instance: {}\nobject_type: {}\nsensor type: {}\nsensor type id: {}\nunit: {}\nunit id: {}",
            self.id, self.name, self.multiplier, self.object_instance, self.object_type, self.sensor_type, self.sensor_type_id, self.unit, self.unit_id
        )
    }
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
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
            "id: {}\ndiscription: {}\nunit id: {}\nunit: {}",
            self.sensor_type_id, self.sensor_description, self.unit_id, self.unit_description
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueMapping {
    text: String,
    value: usize
}

impl fmt::Display for ValueMapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "text: {}, value: {}",
            self.text, self.value
        )
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetIpNonNumericSensor {
    id: String,
    name: String,
    #[serde(alias = "objectType")]
    object_type: String,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "valueMapping")]
    value_mapping: Vec<ValueMapping>
}

impl fmt::Display for BacnetIpNonNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sensor_header = format!(
            "id: {}\nname: {}\nobject type: {}\nsensor type: {}\nsensor type id: {}",
            self.id, self.name, self.object_type, self.sensor_type, self.sensor_type_id
        );
        let sensor_value_mapping = &self.value_mapping.iter().fold(String::new(), |acc, m| {acc + "\n" + &m.to_string()});

        write!(f, "{}\n{}", sensor_header, sensor_value_mapping)
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

pub fn get_bacnet_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
) -> Result<Vec<BacnetIpNumericSensor>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!(
        "{}{}/bacnetIpNumericSensors/{}",
        config.instance_url, BACNET_API_PREFIX, definition_id
    );

    // Start http client
    let req = reqwest::blocking::Client::new();

    // Get response
    let resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()?
        .json::<Vec<BacnetIpNumericSensor>>()?;

    Ok(resp)
}

pub fn get_bacnet_non_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
) -> Result<Vec<BacnetIpNonNumericSensor>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!(
        "{}{}/bacnetIpNonNumericSensors/{}",
        config.instance_url, BACNET_API_PREFIX, definition_id
    );

    // Start http client
    let req = reqwest::blocking::Client::new();

    // Get response
    let resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()?
        .json::<Vec<BacnetIpNonNumericSensor>>()?;

    Ok(resp)
}

pub fn add_bacnet_definition(
    config: &AppConfig,
    name: String,
    asset_type: String,
) -> Result<Value> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!("{}{}", config.instance_url, BACNET_API_PREFIX);

    // Start http client
    let req = reqwest::blocking::Client::new();

    // Construct definition
    let def = BacnetDefinition {
        name,
        asset_type,
        ..Default::default()
    };
    // Get response
    let resp = req
        .post(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .json(&def)
        .send()?
        .json::<Value>()?;

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

pub fn add_or_update_numeric_sensor(
    config: &AppConfig,
    definition_id: String,
    filename: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(sensor)) = reader
        .deserialize::<BacnetIpNumericSensor>()
        .into_iter()
        .next()
    {
        info!("Processing input line: {:?}", sensor);

        match Uuid::try_parse(&sensor.id) {
            Ok(u) => {
                // existing sensor with valid uuid
                println!("Updating sensor with id: {} and name: {}", u, &sensor.name);

                let target_url = format!(
                    "{}{}/bacnetIpNumericSensors/{}/{}",
                    config.instance_url, BACNET_API_PREFIX, definition_id, u
                );

                let resp = req
                    .put(target_url)
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .json(&sensor)
                    .send()?
                    .status();

                println!("server respone: {:#?}", resp);
            }

            Err(e) => {
                if &sensor.name.len() > &0 && &sensor.id == &"".to_string() {
                    println!("Adding new sensor");
                    let target_url = format!(
                        "{}{}/bacnetIpNumericSensors/{}",
                        config.instance_url, BACNET_API_PREFIX, definition_id
                    );

                    let resp = req
                        .post(target_url)
                        .header(AUTHORIZATION, auth_header.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCEPT, "application/json")
                        .json(&sensor)
                        .send()?
                        .status();

                    println!("server respone: {:#?}", resp);
                } else {
                    error!("Error parsing provided sensor id: {}", e);
                }
            }
        }
    }

    Ok(())
}