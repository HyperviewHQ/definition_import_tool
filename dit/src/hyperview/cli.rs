use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use csv::Writer;
use log::{error, LevelFilter};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::Path;

use crate::hyperview::app_errors::AppError;

const ASSET_TYPES: [&str; 29] = [
    "BladeEnclosure",
    "BladeNetwork",
    "BladeServer",
    "BladeStorage",
    "Busway",
    "Camera",
    "Chiller",
    "Crac",
    "Crah",
    "Environmental",
    "FireControlPanel",
    "Generator",
    "InRowCooling",
    "KvmSwitch",
    "Location",
    "Monitor",
    "NetworkDevice",
    "NetworkStorage",
    "NodeServer",
    "PatchPanel",
    "PduAndRpp",
    "PowerMeter",
    "Rack",
    "RackPdu",
    "Server",
    "SmallUps",
    "TransferSwitch",
    "Ups",
    "VirtualServer",
];

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub auth_url: String,
    pub token_url: String,
    pub instance_url: String,
}

pub fn get_config_path() -> String {
    let home_path = dirs::home_dir().expect("Error: Home directory not found");

    format!("{}/.hyperview/hyperview.toml", home_path.to_str().unwrap())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct AppArgs {
    #[arg(short = 'l', long, help = "Debug level", default_value = "error", value_parser(["trace", "debug", "info", "warn", "error"]))]
    pub debug_level: String,

    #[command(subcommand)]
    pub command: LoaderCommands,
}

#[derive(Subcommand)]
pub enum LoaderCommands {
    /// List current BACnet definitions
    GetBacnetDefinitions,

    /// Add a new BACnet definition
    AddBacnetDefinition(AddDefinitionArgs),

    /// Get a list of existing numeric sensors for a specific definition
    GetBacnetNumericSensors(GetSensorsArgs),

    /// Get a list of existing non-numeric sensors for a specific definition
    GetBacnetNonNumericSensors(GetSensorsArgs),

    /// Adds numeric sensors to a definition
    AddBacnetNumericSensor(AddSensorArgs),

    /// Adds non-numeric sensors to a definition
    AddBacnetNonNumericSensor(AddSensorArgs),

    /// List current Modbus definitions
    GetModbusDefinitions,

    /// Add a new Modbus definition
    AddModbusDefinition(AddDefinitionArgs),

    /// Get sensor types compatible with an asset type
    GetSensorTypes(GetSensorTypesArgs),
}

#[derive(Args)]
pub struct AddDefinitionArgs {
    #[arg(short, long, help = "Definition name")]
    pub name: String,

    #[arg(
        short = 't',
        long,
        help = "Asset type. e.g. Crah",
        value_parser(ASSET_TYPES)
    )]
    pub asset_type: String,
}

#[derive(Args)]
pub struct GetSensorsArgs {
    #[arg(short, long, help = "Definition id")]
    pub definition_id: String,

    #[arg(short, long, help = "Select output type. E.g. csv", default_value = "record", value_parser(["record", "csv"]))]
    pub output_type: String,

    #[arg(short, long, help = "output filename. E.g. output.csv")]
    pub filename: Option<String>,
}

#[derive(Args)]
pub struct AddSensorArgs {
    #[arg(short, long, help = "CSV file name")]
    pub filename: String,

    #[arg(short, long, help = "Definition id")]
    pub definition_id: String,
}

#[derive(Args)]
pub struct GetSensorTypesArgs {
    #[arg(
        short = 't',
        long,
        help = "Asset type. e.g. Crah",
        value_parser(ASSET_TYPES)
    )]
    pub asset_type: String,

    #[arg(short, long, help = "Sensor class. E.g. numeric", default_value = "numeric", value_parser(["numeric", "enum"]))]
    pub sensor_class: String,

    #[arg(short, long, help = "Select output type. E.g. csv", default_value = "record", value_parser(["record", "csv"]))]
    pub output_type: String,

    #[arg(short, long, help = "output filename. E.g. output.csv")]
    pub filename: Option<String>,
}

pub fn get_debug_filter(debug_level: &String) -> LevelFilter {
    if debug_level == "error" {
        LevelFilter::Error
    } else if debug_level == "warn" {
        LevelFilter::Warn
    } else if debug_level == "debug" {
        LevelFilter::Debug
    } else if debug_level == "trace" {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    }
}

pub fn write_output<T: Serialize>(filename: String, object_list: Vec<T>) -> Result<()> {
    let mut writer = Writer::from_path(filename)?;

    for object in object_list {
        writer.serialize(object)?;
    }

    Ok(())
}

pub fn handle_output_choice<T: Display + Serialize>(
    output_type: String,
    filename: Option<String>,
    resp: Vec<T>,
) -> Result<()> {
    if output_type == *"csv" {
        if filename.is_none() {
            error!("Must provide a filename. exiting ...");
            return Err(AppError::NoOutputFilename.into());
        } else if let Some(f) = filename {
            if Path::new(&f).exists() {
                error!("Specified file already exists. exiting ...");
                return Err(AppError::FileExists.into());
            }

            write_output(f, resp)?;
        }
    } else {
        for (i, s) in resp.iter().enumerate() {
            println!("---- [{}] ----", i);
            println!("{}\n", s);
        }
    }
    Ok(())
}
