use anyhow::Result;
use clap::Parser;
use hyperview::cli::{get_config_path, get_debug_filter, AppArgs, AppConfig};
use log::{error, info};
use std::path::Path;

use crate::hyperview::{
    api::{get_bacnet_definition_list, get_sensor_type_asset_type_map},
    app_errors::AppError,
    cli::LoaderCommands,
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
            let bacnet_defs = get_bacnet_definition_list(&config)?;

            for (i, def) in bacnet_defs.iter().enumerate() {
                println!("---- [{}] ----", i);
                println!("{}\n", def);
            }
        }

        LoaderCommands::GetSensorTypes(options) => {
            let query = vec![
                ("assetTypeId".to_string(), options.asset_type.clone()),
                (
                    "sensorTypeValueType".to_string(),
                    options.sensor_class.clone(),
                ),
            ];

            let sensor_types = get_sensor_type_asset_type_map(&config, query)?;

            for (i, s) in sensor_types.iter().enumerate() {
                println!("---- [{}] ----", i);
                println!("{}\n", s);
            }
        }

        LoaderCommands::AddBacnetNumeric(options) => {
            let input_file = &options.filename;

            if !Path::new(&input_file).exists() {
                error!("Specified input file does not exists. exiting ...");
                return Err(AppError::InputFileDoesNotExist.into());
            }

            let definition_id = &options.definition_id;

            info!(
                "Uploading numeric sensors using file: {}, for definition: {}",
                input_file, definition_id
            );
        }
    }

    Ok(())
}
