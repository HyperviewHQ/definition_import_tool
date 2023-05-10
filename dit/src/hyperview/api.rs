use anyhow::Result;
use log::{error, info};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use uuid::Uuid;

use super::{api_data::*, auth::get_auth_header, cli::AppConfig};

const BACNET_API_PREFIX: &str = "/api/setting/bacnetIpDefinitions";
const MODBUS_API_PREFIX: &str = "/api/setting/modbusTcpDefinitions";
const SENSOR_TYPE_ASSET_TYPE: &str = "/api/setting/sensorTypeAssetType";

pub fn list_definitions(
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

pub fn list_bacnet_numeric_sensors(
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

pub fn list_modbus_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
) -> Result<Vec<ModbusTcpNumericSensor>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!(
        "{}{}/modbusTcpNumericSensors/{}",
        config.instance_url, MODBUS_API_PREFIX, definition_id
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
        .json::<Vec<ModbusTcpNumericSensor>>()?;

    Ok(resp)
}

pub fn list_bacnet_non_numeric_sensors(
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

pub fn list_modbus_non_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
) -> Result<Vec<ModbusTcpNonNumericSensor>> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // format target
    let target_url = format!(
        "{}{}/modbusTcpNonNumericSensors/{}",
        config.instance_url, MODBUS_API_PREFIX, definition_id
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
        .json::<Vec<ModbusTcpNonNumericSensor>>()?;

    Ok(resp)
}

pub fn add_bacnet_definition(
    config: &AppConfig,
    name: String,
    asset_type: String,
    definition_type: DefinitionType,
) -> Result<Value> {
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

pub fn list_sensor_types(
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

pub fn import_bacnet_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
    filename: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(mut sensor)) = reader.deserialize::<BacnetIpNumericSensor>().next() {
        info!("Processing input line: {:?}", sensor);

        let id = match sensor.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        if sensor.unit_id == Some("".to_string()) {
            sensor.unit_id = None;
        }

        if sensor.unit == Some("".to_string()) {
            sensor.unit = None;
        }

        match Uuid::try_parse(&id) {
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
                    .json::<Value>()?;

                println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
            }

            Err(e) => {
                if !sensor.name.is_empty() && id.is_empty() {
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
                        .json::<Value>()?;

                    println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
                } else {
                    error!("Error parsing provided sensor id: {}", e);
                }
            }
        }
    }

    Ok(())
}

pub fn import_modbus_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
    filename: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(mut sensor)) = reader.deserialize::<ModbusTcpNumericSensor>().next() {
        info!("Processing input line: {:?}", sensor);

        let id = match sensor.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        if sensor.unit_id == Some("".to_string()) {
            sensor.unit_id = None;
        }

        if sensor.unit == Some("".to_string()) {
            sensor.unit = None;
        }

        match Uuid::try_parse(&id) {
            Ok(u) => {
                // existing sensor with valid uuid
                println!("Updating sensor with id: {} and name: {}", u, &sensor.name);

                let target_url = format!(
                    "{}{}/modbusTcpNumericSensors/{}/{}",
                    config.instance_url, MODBUS_API_PREFIX, definition_id, u
                );

                let resp = req
                    .put(target_url)
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .json(&sensor)
                    .send()?
                    .json::<Value>()?;

                println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
            }

            Err(e) => {
                if !sensor.name.is_empty() && id.is_empty() {
                    println!("Adding new sensor with name: {}", &sensor.name);
                    let target_url = format!(
                        "{}{}/modbusTcpNumericSensors/{}",
                        config.instance_url, MODBUS_API_PREFIX, definition_id
                    );

                    let resp = req
                        .post(target_url)
                        .header(AUTHORIZATION, auth_header.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCEPT, "application/json")
                        .json(&sensor)
                        .send()?
                        .json::<Value>()?;

                    println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
                } else {
                    error!("Error parsing provided sensor id: {}", e);
                }
            }
        }
    }

    Ok(())
}

pub fn import_bacnet_non_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
    filename: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(sensor_csv)) = reader.deserialize::<BacnetIpNonNumericSersorCsv>().next() {
        info!("Processing input line: {:?}", sensor_csv);
        let mut sensor: BacnetIpNonNumericSensor = sensor_csv.into();

        let id = match sensor.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        if String::is_empty(&id) {
            sensor.id = None;
        }

        match Uuid::try_parse(&id) {
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
                    .json::<Value>()?;

                println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
            }

            Err(e) => {
                if !sensor.name.is_empty() && id.is_empty() {
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
                        .json::<Value>()?;

                    println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
                } else {
                    error!("Error parsing provided sensor id: {}", e);
                }
            }
        }
    }

    Ok(())
}

pub fn import_modbus_non_numeric_sensors(
    config: &AppConfig,
    definition_id: String,
    filename: String,
) -> Result<()> {
    // Get Authorization header for request
    let auth_header = get_auth_header(config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(sensor_csv)) = reader.deserialize::<ModbusTcpNonNumericSensorCsv>().next() {
        info!("Processing input line: {:?}", sensor_csv);
        let mut sensor: ModbusTcpNonNumericSensor = sensor_csv.into();

        let id = match sensor.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        if String::is_empty(&id) {
            sensor.id = None;
        }

        match Uuid::try_parse(&id) {
            Ok(u) => {
                // existing sensor with valid uuid
                println!("Updating sensor with id: {} and name: {}", u, &sensor.name);

                let target_url = format!(
                    "{}{}/modbusTcpNonNumericSensors/{}/{}",
                    config.instance_url, MODBUS_API_PREFIX, definition_id, u
                );

                let resp = req
                    .put(target_url)
                    .header(AUTHORIZATION, auth_header.clone())
                    .header(CONTENT_TYPE, "application/json")
                    .header(ACCEPT, "application/json")
                    .json(&sensor)
                    .send()?
                    .json::<Value>()?;

                println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
            }

            Err(e) => {
                if !sensor.name.is_empty() && id.is_empty() {
                    println!("Adding new sensor with name: {}", &sensor.name);
                    let target_url = format!(
                        "{}{}/modbusTcpNonNumericSensors/{}",
                        config.instance_url, MODBUS_API_PREFIX, definition_id
                    );

                    let resp = req
                        .post(target_url)
                        .header(AUTHORIZATION, auth_header.clone())
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCEPT, "application/json")
                        .json(&sensor)
                        .send()?
                        .json::<Value>()?;

                    println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
                } else {
                    error!("Error parsing provided sensor id: {}", e);
                }
            }
        }
    }

    Ok(())
}
