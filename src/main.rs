use anyhow::Result;
use clap::Parser;
use hyperview::cli::{get_config_path, get_debug_filter, AppArgs, AppConfig};
use log::{error, info};
use std::path::Path;

use crate::hyperview::app_errors::AppError;

mod hyperview;

fn main() -> Result<()> {
    let args = AppArgs::parse();

    let debug_level = args.debug_level;
    let input_file = args.input_file;

    let level_filter = get_debug_filter(&debug_level);
    env_logger::builder().filter(None, level_filter).init();

    info!("Starting BACnet definition import");
    info!(
        "\nStartup options:\n| debug level: {} |input file: {} |\n",
        debug_level, input_file
    );

    if !Path::new(&input_file).exists() {
        error!("Specified input file does not exists. exiting ...");
        return Err(AppError::InputFileDoesNotExist.into());
    }

    let config: AppConfig = confy::load_path(get_config_path())?;
    info!("{:?}", config);

    Ok(())
}
