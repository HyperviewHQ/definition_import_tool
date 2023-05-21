use anyhow::Result;
use log::{error, info};
use reqwest::{
    blocking::Client,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::{api_data::*, cli::AppConfig};

const BACNET_API_PREFIX: &str = "/api/setting/bacnetIpDefinitions";
const MODBUS_API_PREFIX: &str = "/api/setting/modbusTcpDefinitions";
const SENSOR_TYPE_ASSET_TYPE: &str = "/api/setting/sensorTypeAssetType";

pub fn list_definitions(
    config: &AppConfig,
    definition_type: DefinitionType,
    auth_header: String,
    req: Client,
) -> Result<Vec<Definition>> {
    // format target
    let target_url = match definition_type {
        DefinitionType::Bacnet => {
            format!("{}{}", config.instance_url, BACNET_API_PREFIX)
        }
        DefinitionType::Modbus => {
            format!("{}{}", config.instance_url, MODBUS_API_PREFIX)
        }
    };

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

pub fn list_sensors<T: Serialize + DeserializeOwned + GenericSensor>(
    config: &AppConfig,
    definition_type: DefinitionType,
    definition_data_type: DefinitionDataType,
    definition_id: String,
    auth_header: String,
    req: Client,
    resp: &mut Vec<T>,
) -> Result<()> {
    // format target
    let target_url = match definition_type {
        DefinitionType::Bacnet => match definition_data_type {
            DefinitionDataType::Numeric => {
                format!(
                    "{}{}/bacnetIpNumericSensors/{}",
                    config.instance_url, BACNET_API_PREFIX, definition_id
                )
            }
            DefinitionDataType::NonNumeric => {
                format!(
                    "{}{}/bacnetIpNonNumericSensors/{}",
                    config.instance_url, BACNET_API_PREFIX, definition_id
                )
            }
        },
        DefinitionType::Modbus => match definition_data_type {
            DefinitionDataType::Numeric => {
                format!(
                    "{}{}/modbusTcpNumericSensors/{}",
                    config.instance_url, MODBUS_API_PREFIX, definition_id
                )
            }
            DefinitionDataType::NonNumeric => {
                format!(
                    "{}{}/modbusTcpNonNumericSensors/{}",
                    config.instance_url, MODBUS_API_PREFIX, definition_id
                )
            }
        },
    };

    // Get response
    *resp = req
        .get(target_url)
        .header(AUTHORIZATION, auth_header)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()?
        .json::<Vec<T>>()?;

    Ok(())
}

pub fn add_definition(
    config: &AppConfig,
    name: String,
    asset_type: String,
    definition_type: DefinitionType,
    auth_header: String,
    req: Client,
) -> Result<Value> {
    // format target
    let target_url = match definition_type {
        DefinitionType::Bacnet => {
            format!("{}{}", config.instance_url, BACNET_API_PREFIX)
        }
        DefinitionType::Modbus => {
            format!("{}{}", config.instance_url, MODBUS_API_PREFIX)
        }
    };

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
    auth_header: String,
    req: Client,
) -> Result<Vec<SensorType>> {
    // format target
    let target_url = format!("{}{}", config.instance_url, SENSOR_TYPE_ASSET_TYPE);

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
    auth_header: String,
    req: Client,
) -> Result<()> {
    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(mut sensor)) = reader.deserialize::<BacnetIpNumericSensor>().next() {
        info!("Processing input line: {:?}", sensor);

        let id = sensor.get_id_as_string();
        sensor.clean_empty_id();
        sensor.clean_sensor_empty_unit();

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
    auth_header: String,
    req: Client,
) -> Result<()> {
    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(mut sensor)) = reader.deserialize::<ModbusTcpNumericSensor>().next() {
        info!("Processing input line: {:?}", sensor);

        let id = sensor.get_id_as_string();
        sensor.clean_empty_id();
        sensor.clean_sensor_empty_unit();

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
    auth_header: String,
    req: Client,
) -> Result<()> {
    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(sensor_csv)) = reader.deserialize::<BacnetIpNonNumericSersorCsv>().next() {
        info!("Processing input line: {:?}", sensor_csv);
        let mut sensor: BacnetIpNonNumericSensor = sensor_csv.into();

        let id = sensor.get_id_as_string();
        sensor.clean_empty_id();

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
    auth_header: String,
    req: Client,
) -> Result<()> {
    let mut reader = csv::Reader::from_path(filename)?;

    while let Some(Ok(sensor_csv)) = reader.deserialize::<ModbusTcpNonNumericSensorCsv>().next() {
        info!("Processing input line: {:?}", sensor_csv);
        let mut sensor: ModbusTcpNonNumericSensor = sensor_csv.into();

        let id = sensor.get_id_as_string();
        sensor.clean_empty_id();

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
