use anyhow::Result;
use clap::Parser;
use log::{error, info};
use std::path::Path;

use crate::hyperview::{
    api::{
        add_definition, import_bacnet_non_numeric_sensors, import_bacnet_numeric_sensors,
        import_modbus_non_numeric_sensors, import_modbus_numeric_sensors, list_definitions,
        list_sensor_types, list_sensors,
    },
    api_data::{
        BacnetIpNonNumericSensor, BacnetIpNonNumericSensorExportWrapper, BacnetIpNumericSensor,
        DefinitionDataType, DefinitionType, ModbusTcpNonNumericSensor,
        ModbusTcpNonNumericSensorExportWrapper, ModbusTcpNumericSensor,
    },
    app_errors::AppError,
    auth::get_auth_header,
    cli::{
        get_config_path, get_debug_filter, handle_output_choice, AppArgs, AppConfig, LoaderCommands,
    },
};

mod hyperview;

fn main() -> Result<()> {
    let args = AppArgs::parse();

    let debug_level = args.debug_level;

    let level_filter = get_debug_filter(&debug_level);
    env_logger::builder().filter(None, level_filter).init();

    info!("Starting BACnet definition import");
    info!("Startup options:\n| debug level: {} |\n", debug_level);

    let config: AppConfig = confy::load_path(get_config_path())?;
    info!("Hyperview Instance: {}", config.instance_url);

    // Get Authorization header for request
    let auth_header = get_auth_header(&config)?;

    // Start http client
    let req = reqwest::blocking::Client::new();

    match &args.command {
        LoaderCommands::ListBacnetDefinitions => {
            let resp = list_definitions(&config, DefinitionType::Bacnet, auth_header, req)?;

            for (i, d) in resp.iter().enumerate() {
                println!("---- [{}] ----", i);
                println!("{}\n", d);
            }
        }

        LoaderCommands::AddBacnetDefinition(options) => {
            let resp = add_definition(
                &config,
                options.name.clone(),
                options.asset_type.clone(),
                DefinitionType::Bacnet,
                auth_header,
                req,
            )?;

            println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
        }

        LoaderCommands::ListBacnetNumericSensors(options) => {
            let mut resp: Vec<BacnetIpNumericSensor> = Vec::new();
            list_sensors(
                &config,
                DefinitionType::Bacnet,
                DefinitionDataType::Numeric,
                options.definition_id.clone(),
                auth_header,
                req,
                &mut resp,
            )?;
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp)?;
        }

        LoaderCommands::ListBacnetNonNumericSensors(options) => {
            let mut resp: Vec<BacnetIpNonNumericSensor> = Vec::new();
            list_sensors(
                &config,
                DefinitionType::Bacnet,
                DefinitionDataType::NonNumeric,
                options.definition_id.clone(),
                auth_header,
                req,
                &mut resp,
            )?;
            let resp_export_do: Vec<BacnetIpNonNumericSensorExportWrapper> = resp
                .into_iter()
                .map(BacnetIpNonNumericSensorExportWrapper)
                .collect();
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp_export_do)?;
        }

        LoaderCommands::ImportBacnetNumericSensors(options) => {
            let filename = &options.filename;

            if !Path::new(filename).exists() {
                error!("Specified input file does not exists. exiting ...");
                return Err(AppError::InputFileDoesNotExist.into());
            }

            let definition_id = &options.definition_id;

            info!(
                "Uploading numeric sensors using file: {}, for definition: {}",
                filename, definition_id
            );

            import_bacnet_numeric_sensors(
                &config,
                definition_id.to_owned(),
                filename.to_owned(),
                auth_header,
                req,
            )?;
        }

        LoaderCommands::ImportBacnetNonNumericSensors(options) => {
            let filename = &options.filename;

            if !Path::new(filename).exists() {
                error!("Specified input file does not exists. exiting ...");
                return Err(AppError::InputFileDoesNotExist.into());
            }

            let definition_id = &options.definition_id;

            info!(
                "Uploading numeric sensors using file: {}, for definition: {}",
                filename, definition_id
            );

            import_bacnet_non_numeric_sensors(
                &config,
                definition_id.to_owned(),
                filename.to_owned(),
                auth_header,
                req,
            )?;
        }

        LoaderCommands::ListModbusDefinitions => {
            let resp = list_definitions(&config, DefinitionType::Modbus, auth_header, req)?;

            for (i, d) in resp.iter().enumerate() {
                println!("---- [{}] ----", i);
                println!("{}\n", d);
            }
        }

        LoaderCommands::AddModbusDefinition(options) => {
            let resp = add_definition(
                &config,
                options.name.clone(),
                options.asset_type.clone(),
                DefinitionType::Modbus,
                auth_header,
                req,
            )?;

            println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
        }

        LoaderCommands::ListModbusNumericSensors(options) => {
            let mut resp: Vec<ModbusTcpNumericSensor> = Vec::new();
            list_sensors(
                &config,
                DefinitionType::Modbus,
                DefinitionDataType::Numeric,
                options.definition_id.clone(),
                auth_header,
                req,
                &mut resp,
            )?;
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp)?;
        }

        LoaderCommands::ListModbusNonNumericSensors(options) => {
            let mut resp: Vec<ModbusTcpNonNumericSensor> = Vec::new();
            list_sensors(
                &config,
                DefinitionType::Modbus,
                DefinitionDataType::NonNumeric,
                options.definition_id.clone(),
                auth_header,
                req,
                &mut resp,
            )?;
            let resp_export_do: Vec<ModbusTcpNonNumericSensorExportWrapper> = resp
                .into_iter()
                .map(ModbusTcpNonNumericSensorExportWrapper)
                .collect();
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp_export_do)?;
        }

        LoaderCommands::ImportModbusNumericSensors(options) => {
            let filename = &options.filename;

            if !Path::new(filename).exists() {
                error!("Specified input file does not exists. exiting ...");
                return Err(AppError::InputFileDoesNotExist.into());
            }

            let definition_id = &options.definition_id;

            info!(
                "Uploading numeric sensors using file: {}, for definition: {}",
                filename, definition_id
            );

            import_modbus_numeric_sensors(
                &config,
                definition_id.to_owned(),
                filename.to_owned(),
                auth_header,
                req,
            )?;
        }

        LoaderCommands::ImportModbusNonNumericSensors(options) => {
            let filename = &options.filename;

            if !Path::new(filename).exists() {
                error!("Specified input file does not exists. exiting ...");
                return Err(AppError::InputFileDoesNotExist.into());
            }

            let definition_id = &options.definition_id;

            info!(
                "Uploading numeric sensors using file: {}, for definition: {}",
                filename, definition_id
            );

            import_modbus_non_numeric_sensors(
                &config,
                definition_id.to_owned(),
                filename.to_owned(),
                auth_header,
                req,
            )?;
        }

        LoaderCommands::ListSensorTypes(options) => {
            let query = vec![
                ("assetTypeId".to_string(), options.asset_type.clone()),
                (
                    "sensorTypeValueType".to_string(),
                    options.sensor_class.clone(),
                ),
            ];

            let resp = list_sensor_types(&config, query, auth_header, req)?;
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp)?;
        }
    }

    Ok(())
}
