use anyhow::Result;
use clap::Parser;
use log::{error, info};
use std::path::Path;

use crate::hyperview::{
    api::{
        add_bacnet_definition, add_or_update_numeric_sensor, get_bacnet_definition_list,
        get_bacnet_non_numeric_sensors, get_bacnet_numeric_sensors, get_sensor_type_asset_type_map,
    },
    app_errors::AppError,
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

    match &args.command {
        LoaderCommands::GetBacnetDefinitions => {
            let resp = get_bacnet_definition_list(&config)?;

            for (i, d) in resp.iter().enumerate() {
                println!("---- [{}] ----", i);
                println!("{}\n", d);
            }
        }

        LoaderCommands::AddBacnetDefinition(options) => {
            let resp =
                add_bacnet_definition(&config, options.name.clone(), options.asset_type.clone())?;

            println!("server respone: {}", serde_json::to_string_pretty(&resp)?);
        }

        LoaderCommands::GetBacnetNumericSensors(options) => {
            let resp = get_bacnet_numeric_sensors(&config, options.definition_id.clone())?;
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp)?;
        }

        LoaderCommands::GetBacnetNonNumericSensors(options) => {
            let resp = get_bacnet_non_numeric_sensors(&config, options.definition_id.clone())?;
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp)?;
        }

        LoaderCommands::AddBacnetNumericSensor(options) => {
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

            add_or_update_numeric_sensor(&config, definition_id.to_owned(), filename.to_owned())?;
        }

        LoaderCommands::GetSensorTypes(options) => {
            let query = vec![
                ("assetTypeId".to_string(), options.asset_type.clone()),
                (
                    "sensorTypeValueType".to_string(),
                    options.sensor_class.clone(),
                ),
            ];

            let resp = get_sensor_type_asset_type_map(&config, query)?;
            let filename = &options.filename;
            let output_type = &options.output_type;

            handle_output_choice(output_type.to_owned(), filename.to_owned(), resp)?;
        }
    }

    Ok(())
}
