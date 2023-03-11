use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

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
    #[arg(short, long, help = "Debug level", default_value = "error", value_parser(["trace", "debug", "info", "warn", "error"]))]
    pub debug_level: String,

    #[command(subcommand)]
    pub command: LoaderCommands,
}

#[derive(Subcommand)]
pub enum LoaderCommands {
    /// List current BACnet definitions
    GetBacnetDefinitions,

    /// Add a new BACnet definition
    AddBacnetDefinition(AddBacnetDefinitionArgs),

    /// Get a list of existing numeric sensors for a specific definintion
    GetBacnetNumericSensors(GetBacnetNumericSensorsArgs),

    /// Adds numeric sensor definitions to a definition
    AddBacnetNumeric(AddBacnetNumericArgs),

    /// Get sensor types compatible with an asset type
    GetSensorTypes(GetSensorTypesArgs),
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
}

#[derive(Args)]
pub struct AddBacnetNumericArgs {
    #[arg(short, long, help = "CSV file name")]
    pub filename: String,

    #[arg(short, long, help = "Definition id")]
    pub definition_id: String,
}

#[derive(Args)]
pub struct AddBacnetDefinitionArgs {
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
pub struct GetBacnetNumericSensorsArgs {
    #[arg(short, long, help = "Definition id")]
    pub definition_id: String,
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
