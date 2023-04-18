use anyhow::Result;
use log::{error, info};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use uuid::Uuid;

use super::{api_data::*, auth::get_auth_header, cli::AppConfig};

const BACNET_API_PREFIX: &str = "/api/setting/bacnetIpDefinitions";
const MODBUS_API_PREFIX: &str = "/api/setting/modbusTcpDefinitions";
const SENSOR_TYPE_ASSET_TYPE: &str = "/api/setting/sensorTypeAssetType";

pub fn get_bacnet_definition_list(
    config: &AppConfig,
    definition_type: DefinitionType,
) -> Result<Vec<Definition>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = match definition_type {
        DefinitionType::Bacnet => {
            format!("{}{}", config.instance_url, BACNET_API_PREFIX)
        }
        DefinitionType::Modbus => {
            format!("{}{}", config.instance_url, MODBUS_API_PREFIX)
        }
    };

    // Start http client
    let req = reqwest::blocking::Client::new();

    // Get response
    let resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()?
        .json::<Vec<Definition>>()?;

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
    let def = Definition {
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
                    println!("Adding new sensor with name: {}", &sensor.name);
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

pub fn add_or_update_non_numeric_sensor(
    config: &AppConfig,
    definition_id: String,
    filename: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(sensor_csv)) = reader
        .deserialize::<BacnetIpNonNumericSersorCsv>()
        .into_iter()
        .next()
    {
        info!("Processing input line: {:?}", sensor_csv);
        let sensor: BacnetIpNonNumericSensor = sensor_csv.into();

        match Uuid::try_parse(&sensor.id) {
            Ok(u) => {
                // existing sensor with valid uuid
                println!("Updating sensor with id: {} and name: {}", u, &sensor.name);

                let target_url = format!(
                    "{}{}/bacnetIpNonNumericSensors/{}/{}",
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
                    println!("Adding new sensor with name: {}", &sensor.name);
                    let target_url = format!(
                        "{}{}/bacnetIpNonNumericSensors/{}",
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
