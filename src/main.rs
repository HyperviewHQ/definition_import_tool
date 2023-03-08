use anyhow::Result;
use clap::Parser;
use hyperview::cli::{get_config_path, get_debug_filter, AppArgs, AppConfig};
use log::info;

use crate::hyperview::{api::get_bacnet_definition_list, app_errors::AppError};

mod hyperview;

fn main() -> Result<()> {
    let args = AppArgs::parse();

    let debug_level = args.debug_level;

    let level_filter = get_debug_filter(&debug_level);
    env_logger::builder().filter(None, level_filter).init();

    info!("Starting BACnet definition import");
    info!("Startup options:\n| debug level: {} |\n", debug_level);

    /*
        if !Path::new(&input_file).exists() {
            error!("Specified input file does not exists. exiting ...");
            return Err(AppError::InputFileDoesNotExist.into());
        }
    */
    let config: AppConfig = confy::load_path(get_config_path())?;
    info!("Connecting to: {}", config.instance_url);

    let bacnet_defs = get_bacnet_definition_list(&config)?;

    for def in bacnet_defs {
        info!("{}", def);
    }

    Ok(())
}
