use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use csv::Writer;
use log::{error, LevelFilter};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::path::{Path, MAIN_SEPARATOR_STR};

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

    format!(
        "{}{}.hyperview{}hyperview.toml",
        home_path.to_str().unwrap(),
        MAIN_SEPARATOR_STR,
        MAIN_SEPARATOR_STR
    )
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
    ListBacnetDefinitions,

    /// Add a new BACnet definition
    AddBacnetDefinition(AddDefinitionArgs),

    /// List numeric sensors for a definition
    ListBacnetNumericSensors(ListSensorsArgs),

    /// List non-numeric sensors for a definition
    ListBacnetNonNumericSensors(ListSensorsArgs),

    /// Import numeric sensors to a definition
    ImportBacnetNumericSensors(ImportSensorArgs),

    /// Import non-numeric sensors to a definition
    ImportBacnetNonNumericSensors(ImportSensorArgs),

    /// List current Modbus definitions
    ListModbusDefinitions,

    /// Add a new Modbus definition
    AddModbusDefinition(AddDefinitionArgs),

    /// List numeric sensors for a definition
    ListModbusNumericSensors(ListSensorsArgs),

    /// List non-numeric sensors for a definition
    ListModbusNonNumericSensors(ListSensorsArgs),

    /// Import numeric sensors to a definition
    ImportModbusNumericSensors(ImportSensorArgs),

    /// Import non-numeric sensors to a definition
    ImportModbusNonNumericSensors(ImportSensorArgs),

    /// List sensor types compatible with an asset type
    ListSensorTypes(ListSensorTypesArgs),
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
pub struct ListSensorsArgs {
    #[arg(short, long, help = "Definition id")]
    pub definition_id: String,

    #[arg(short, long, help = "Select output type. E.g. csv", default_value = "record", value_parser(["record", "csv"]))]
    pub output_type: String,

    #[arg(short, long, help = "output filename. E.g. output.csv")]
    pub filename: Option<String>,
}

#[derive(Args)]
pub struct ImportSensorArgs {
    #[arg(short, long, help = "CSV file name")]
    pub filename: String,

    #[arg(short, long, help = "Definition id")]
    pub definition_id: String,
}

#[derive(Args)]
pub struct ListSensorTypesArgs {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;
    use std::path::MAIN_SEPARATOR_STR;
    use tempfile::NamedTempFile;

    #[test]
    fn test_get_config_path() {
        let config_path = get_config_path();
        let home_path = dirs::home_dir().unwrap();
        let expected_path = format!(
            "{}{}.hyperview{}hyperview.toml",
            home_path.to_str().unwrap(),
            MAIN_SEPARATOR_STR,
            MAIN_SEPARATOR_STR
        );

        assert_eq!(config_path, expected_path);
    }

    #[test]
    fn test_write_output() {
        // Create test data
        let data = vec![1, 2, 3, 4, 5];

        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_str().unwrap().to_string();

        // Call the function with the test data and the temporary file path
        let result = write_output(temp_file_path.clone(), data);
        assert!(result.is_ok());

        // Read back the file
        let file = File::open(temp_file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();

        assert_eq!("1\n2\n3\n4\n5\n", contents);
    }

    #[test]
    fn test_handle_output_choice_no_filename() {
        let output_type = "csv".to_string();
        let filename = None;
        let resp: Vec<i32> = vec![1, 2, 3, 4, 5];

        match handle_output_choice(output_type, filename, resp) {
            Err(e) => assert_eq!(e.to_string(), AppError::NoOutputFilename.to_string()),
            _ => panic!("Expected Err, but got Ok"),
        }
    }

    #[test]
    fn test_handle_output_choice_file_exists() {
        let output_type = "csv".to_string();
        let temp_file = NamedTempFile::new().unwrap();
        let filename = Some(temp_file.path().to_str().unwrap().to_string());
        let resp: Vec<i32> = vec![1, 2, 3, 4, 5];

        match handle_output_choice(output_type, filename, resp) {
            Err(e) => assert_eq!(e.to_string(), AppError::FileExists.to_string()),
            _ => panic!("Expected Err, but got Ok"),
        }
    }

    #[test]
    fn test_handle_output_choice_write_output() {
        let output_type = "csv".to_string();
        let temp_file = NamedTempFile::new().unwrap();
        let temp_file_path = temp_file.path().to_str().unwrap().to_string();
        let filename = Some(temp_file_path.clone() + "_new");
        let resp: Vec<i32> = vec![1, 2, 3, 4, 5];

        let result = handle_output_choice(output_type, filename.clone(), resp);
        assert!(result.is_ok());

        let mut file = File::open(filename.unwrap()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Check the contents of the file
        assert_eq!(contents, "1\n2\n3\n4\n5\n");
    }
}
