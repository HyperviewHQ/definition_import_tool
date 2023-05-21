use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};
use std::fmt;

// marker trait
pub trait NumericSensor {
    fn clean_sensor_empty_unit(&mut self);
}

pub trait GenericSensor {
    fn get_id_as_string(&self) -> String;
    fn clean_empty_id(&mut self);
}

#[derive(Debug)]
pub enum DefinitionType {
    Bacnet,
    Modbus,
}

#[derive(Debug)]
pub enum DefinitionDataType {
    Numeric,
    NonNumeric,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub id: Option<String>,
    pub name: String,
    #[serde(alias = "assetType")]
    pub asset_type: String,
    #[serde(alias = "associatedAssets")]
    pub associated_assets: usize,
}

impl fmt::Display for Definition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        write!(
            f,
            "id: {}\nname: {}\nasset type: {}\nassociated_assets: {}",
            id, self.name, self.asset_type, self.associated_assets
        )
    }
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetIpNumericSensor {
    pub id: Option<String>,
    pub name: String,
    multiplier: f64,
    #[serde(alias = "objectInstance")]
    object_instance: usize,
    #[serde(alias = "objectType")]
    object_type: String,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub unit: Option<String>,
    #[serde(alias = "unitId")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub unit_id: Option<String>,
}

impl fmt::Display for BacnetIpNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        let unit = match self.unit.clone() {
            Some(x) => x,
            None => String::new(),
        };

        let unit_id = match self.unit_id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        write!(
            f,
            "id: {}\nname: {}\nmultiplier: {}\nobject_instance: {}\nobject_type: {}\nsensor type: {}\nsensor type id: {}\nunit: {}\nunit id: {}",
            id, self.name, self.multiplier, self.object_instance, self.object_type, self.sensor_type, self.sensor_type_id, unit, unit_id
        )
    }
}

impl GenericSensor for BacnetIpNumericSensor {
    fn get_id_as_string(&self) -> String {
        match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        }
    }

    fn clean_empty_id(&mut self) {
        let id = self.get_id_as_string();
        if String::is_empty(&id) {
            self.id = None;
        }
    }
}

impl NumericSensor for BacnetIpNumericSensor {
    fn clean_sensor_empty_unit(&mut self) {
        if self.unit_id == Some("".to_string()) {
            self.unit_id = None;
        }

        if self.unit == Some("".to_string()) {
            self.unit = None;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueMapping {
    text: String,
    value: usize,
}

impl fmt::Display for ValueMapping {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "text: {}, value: {}", self.text, self.value)
    }
}

#[derive(Debug, Deserialize)]
pub struct BacnetIpNonNumericSersorCsv {
    id: String,
    name: String,
    #[serde(alias = "objectInstance")]
    object_instance: usize,
    #[serde(alias = "objectType")]
    object_type: String,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "valueMapping")]
    value_mapping: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetIpNonNumericSensor {
    pub id: Option<String>,
    pub name: String,
    #[serde(alias = "objectInstance")]
    object_instance: usize,
    #[serde(alias = "objectType")]
    object_type: String,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "valueMapping")]
    value_mapping: Vec<ValueMapping>,
}

impl fmt::Display for BacnetIpNonNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        let sensor_header = format!(
            "id: {}\nname: {}\nobject type: {}\nsensor type: {}\nsensor type id: {}",
            id, self.name, self.object_type, self.sensor_type, self.sensor_type_id
        );
        let sensor_value_mapping = &self
            .value_mapping
            .iter()
            .fold(String::new(), |acc, m| acc + "\n" + &m.to_string());

        write!(f, "{}\n{}", sensor_header, sensor_value_mapping)
    }
}

impl GenericSensor for BacnetIpNonNumericSensor {
    fn get_id_as_string(&self) -> String {
        match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        }
    }

    fn clean_empty_id(&mut self) {
        let id = self.get_id_as_string();
        if String::is_empty(&id) {
            self.id = None;
        }
    }
}

// The export wrapper is implemented because we have two potential serialization paths.
// One for CSV export and another from the standard serde Serialize/De-Serialize funtionality
pub struct BacnetIpNonNumericSensorExportWrapper(pub BacnetIpNonNumericSensor);

impl fmt::Display for BacnetIpNonNumericSensorExportWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for BacnetIpNonNumericSensorExportWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BacnetIpNonNumericSensorExportWrapper", 7)?;

        state.serialize_field("id", &self.0.id)?;
        state.serialize_field("name", &self.0.name)?;
        state.serialize_field("objectInstance", &self.0.object_instance)?;
        state.serialize_field("objectType", &self.0.object_type)?;
        state.serialize_field("sensorType", &self.0.sensor_type)?;
        state.serialize_field("sensorTypeId", &self.0.sensor_type_id)?;

        let value_mapping_str = self
            .0
            .value_mapping
            .iter()
            .map(|vm| format!("{}:{}", vm.text, vm.value))
            .collect::<Vec<String>>()
            .join(",");

        state.serialize_field("valueMapping", &value_mapping_str)?;

        state.end()
    }
}

impl From<BacnetIpNonNumericSersorCsv> for BacnetIpNonNumericSensor {
    fn from(source: BacnetIpNonNumericSersorCsv) -> Self {
        let mappings = source
            .value_mapping
            .split(',')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, ':');
                Some(ValueMapping {
                    text: parts.next()?.to_string(),
                    value: parts
                        .next()?
                        .parse::<usize>()
                        .expect("could not parse value to integer"),
                })
            })
            .collect();

        BacnetIpNonNumericSensor {
            id: Some(source.id),
            name: source.name,
            object_instance: source.object_instance,
            object_type: source.object_type,
            sensor_type: source.sensor_type,
            sensor_type_id: source.sensor_type_id,
            value_mapping: mappings,
        }
    }
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SensorType {
    #[serde(alias = "abbreviatedUnit")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    _abbreviated_unit: String,
    #[serde(alias = "isManuallyCreatable")]
    _is_manually_creatable: bool,
    #[serde(alias = "minimumValidValue")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    _minimum_valid_value: String,
    #[serde(alias = "sensorDescription")]
    sensor_description: String,
    #[serde(alias = "sensorParentType")]
    _sensor_parent_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "unitDescription")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit_description: String,
    #[serde(alias = "unitId")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit_id: String,
}

impl fmt::Display for SensorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\ndiscription: {}\nunit id: {}\nunit: {}",
            self.sensor_type_id, self.sensor_description, self.unit_id, self.unit_description
        )
    }
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusTcpNumericSensor {
    pub id: Option<String>,
    pub name: String,
    multiplier: f64,
    address: usize,
    #[serde(alias = "registerType")]
    register_type: String,
    #[serde(alias = "dataSetting")]
    data_setting: String,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub unit: Option<String>,
    #[serde(alias = "unitId")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub unit_id: Option<String>,
}

impl fmt::Display for ModbusTcpNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        let unit = match self.unit.clone() {
            Some(x) => x,
            None => String::new(),
        };

        let unit_id = match self.unit_id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        write!(
            f,
            "id: {}\nname: {}\nmultiplier: {}\naddress: {}\nregister_type: {}\ndata setting: {}\nsensor type: {}\nsensor type id: {}\nunit: {}\nunit id: {}",
            id, self.name, self.multiplier, self.address, self.register_type, self.data_setting, self.sensor_type, self.sensor_type_id, unit, unit_id
        )
    }
}

impl GenericSensor for ModbusTcpNumericSensor {
    fn get_id_as_string(&self) -> String {
        match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        }
    }

    fn clean_empty_id(&mut self) {
        let id = self.get_id_as_string();
        if String::is_empty(&id) {
            self.id = None;
        }
    }
}

impl NumericSensor for ModbusTcpNumericSensor {
    fn clean_sensor_empty_unit(&mut self) {
        if self.unit_id == Some("".to_string()) {
            self.unit_id = None;
        }

        if self.unit == Some("".to_string()) {
            self.unit = None;
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusTcpNonNumericSensorCsv {
    pub id: Option<String>,
    pub name: String,
    address: usize,
    #[serde(alias = "dataType")]
    data_type: String,
    #[serde(alias = "registerType")]
    register_type: String,
    #[serde(alias = "startBit")]
    start_bit: usize,
    #[serde(alias = "endBit")]
    end_bit: usize,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "valueMapping")]
    value_mapping: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModbusTcpNonNumericSensor {
    pub id: Option<String>,
    pub name: String,
    address: usize,
    #[serde(alias = "dataType")]
    data_type: String,
    #[serde(alias = "registerType")]
    register_type: String,
    #[serde(alias = "startBit")]
    start_bit: usize,
    #[serde(alias = "endBit")]
    end_bit: usize,
    #[serde(alias = "sensorType")]
    sensor_type: String,
    #[serde(alias = "sensorTypeId")]
    sensor_type_id: String,
    #[serde(alias = "valueMapping")]
    value_mapping: Vec<ValueMapping>,
}

impl fmt::Display for ModbusTcpNonNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        };

        let sensor_header = format!(
            "id: {}\nname: {}\naddress: {}\ndata type: {}\nregister type: {}\nstart bit: {}\nend bit: {}\nsensor type: {}\nsensor type id: {}",
            id, self.name, self.address, self.data_type, self.register_type, self.start_bit, self.end_bit, self.sensor_type, self.sensor_type_id
        );
        let sensor_value_mapping = &self
            .value_mapping
            .iter()
            .fold(String::new(), |acc, m| acc + "\n" + &m.to_string());

        write!(f, "{}\n{}", sensor_header, sensor_value_mapping)
    }
}

impl GenericSensor for ModbusTcpNonNumericSensor {
    fn get_id_as_string(&self) -> String {
        match self.id.clone() {
            Some(x) => x,
            None => String::new(),
        }
    }

    fn clean_empty_id(&mut self) {
        let id = self.get_id_as_string();
        if String::is_empty(&id) {
            self.id = None;
        }
    }
}

// The export wrapper is implemented because we have two potential serialization paths.
// One for CSV export and another from the standard serde Serialize/De-Serialize funtionality
pub struct ModbusTcpNonNumericSensorExportWrapper(pub ModbusTcpNonNumericSensor);

impl fmt::Display for ModbusTcpNonNumericSensorExportWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for ModbusTcpNonNumericSensorExportWrapper {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state =
            serializer.serialize_struct("ModbusTcpNonNumericSensorExportWrapper", 10)?;

        state.serialize_field("id", &self.0.id)?;
        state.serialize_field("name", &self.0.name)?;
        state.serialize_field("address", &self.0.address)?;
        state.serialize_field("dataType", &self.0.data_type)?;
        state.serialize_field("registerType", &self.0.register_type)?;
        state.serialize_field("startBit", &self.0.start_bit)?;
        state.serialize_field("endBit", &self.0.end_bit)?;
        state.serialize_field("sensorType", &self.0.sensor_type)?;
        state.serialize_field("sensorTypeId", &self.0.sensor_type_id)?;

        let value_mapping_str = self
            .0
            .value_mapping
            .iter()
            .map(|vm| format!("{}:{}", vm.text, vm.value))
            .collect::<Vec<String>>()
            .join(",");

        state.serialize_field("valueMapping", &value_mapping_str)?;

        state.end()
    }
}

impl From<ModbusTcpNonNumericSensorCsv> for ModbusTcpNonNumericSensor {
    fn from(source: ModbusTcpNonNumericSensorCsv) -> Self {
        let mappings = source
            .value_mapping
            .split(',')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, ':');
                Some(ValueMapping {
                    text: parts.next()?.to_string(),
                    value: parts
                        .next()?
                        .parse::<usize>()
                        .expect("could not parse value to integer"),
                })
            })
            .collect();

        ModbusTcpNonNumericSensor {
            id: source.id,
            name: source.name,
            address: source.address,
            data_type: source.data_type,
            register_type: source.register_type,
            start_bit: source.start_bit,
            end_bit: source.end_bit,
            sensor_type: source.sensor_type,
            sensor_type_id: source.sensor_type_id,
            value_mapping: mappings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bacnet_sensor_csv_serialization() {
        let sensor = BacnetIpNonNumericSensorExportWrapper(BacnetIpNonNumericSensor {
            id: Some("247a4ad9-9d18-4bf4-b20b-a1d7d61b3971".to_string()),
            name: "Sensor 1".to_string(),
            object_instance: 0,
            object_type: "Temperature".to_string(),
            sensor_type: "Analog".to_string(),
            sensor_type_id: "1000".to_string(),
            value_mapping: vec![
                ValueMapping {
                    text: "Low".to_string(),
                    value: 0,
                },
                ValueMapping {
                    text: "High".to_string(),
                    value: 1,
                },
            ],
        });

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&sensor).expect("Failed to serialize sensor");

        let data = String::from_utf8(wtr.into_inner().expect("Failed to get inner writer"))
            .expect("Failed to convert to string");

        let expected_data = "id,name,objectInstance,objectType,sensorType,sensorTypeId,valueMapping\n247a4ad9-9d18-4bf4-b20b-a1d7d61b3971,Sensor 1,0,Temperature,Analog,1000,\"Low:0,High:1\"\n";

        assert_eq!(data, expected_data);
    }

    #[test]
    fn test_modbus_sensor_csv_serialization() {
        let sensor = ModbusTcpNonNumericSensorExportWrapper(ModbusTcpNonNumericSensor {
            id: Some("ffd733e3-2ee2-4e81-a688-2483cb011698".to_string()),
            name: "Clogged filter 1".to_string(),
            address: 1,
            data_type: "uInteger16".to_string(),
            register_type: "holdingRegister".to_string(),
            start_bit: 1,
            end_bit: 16,
            sensor_type: "cloggedFilter".to_string(),
            sensor_type_id: "f4531ff2-ebf8-49d2-bd4f-4d64c39e4283".to_string(),
            value_mapping: vec![
                ValueMapping {
                    text: "Inactive".to_string(),
                    value: 0,
                },
                ValueMapping {
                    text: "Active".to_string(),
                    value: 1,
                },
            ],
        });

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&sensor).expect("Failed to serialize sensor");

        let data = String::from_utf8(wtr.into_inner().expect("Failed to get inner writer"))
            .expect("Failed to convert to string");

        let expected_data = "id,name,address,dataType,registerType,startBit,endBit,sensorType,sensorTypeId,valueMapping\nffd733e3-2ee2-4e81-a688-2483cb011698,Clogged filter 1,1,uInteger16,holdingRegister,1,16,cloggedFilter,f4531ff2-ebf8-49d2-bd4f-4d64c39e4283,\"Inactive:0,Active:1\"\n";

        assert_eq!(data, expected_data);
    }

    #[test]
    fn test_get_id_as_string() {
        let mut sensor = BacnetIpNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        assert_eq!(
            sensor.get_id_as_string(),
            "13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()
        );

        sensor.id = Some("".to_string());
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        sensor.id = None;
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        let mut sensor = BacnetIpNonNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        assert_eq!(
            sensor.get_id_as_string(),
            "13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()
        );

        sensor.id = Some("".to_string());
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        sensor.id = None;
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        let mut sensor = ModbusTcpNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        assert_eq!(
            sensor.get_id_as_string(),
            "13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()
        );

        sensor.id = Some("".to_string());
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        sensor.id = None;
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        let mut sensor = ModbusTcpNonNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        assert_eq!(
            sensor.get_id_as_string(),
            "13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()
        );

        sensor.id = Some("".to_string());
        assert_eq!(sensor.get_id_as_string(), "".to_string());

        sensor.id = None;
        assert_eq!(sensor.get_id_as_string(), "".to_string());
    }

    #[test]
    fn test_clean_empty_id() {
        let mut sensor = BacnetIpNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        sensor.clean_empty_id();
        assert_eq!(
            sensor.id,
            Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string())
        );

        sensor.id = Some("".to_string());
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        sensor.id = None;
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        sensor.unit = Some("".to_string());
        sensor.unit_id = Some("".to_string());
        sensor.clean_sensor_empty_unit();
        assert_eq!(sensor.unit, None);
        assert_eq!(sensor.unit_id, None);

        sensor.unit = Some("286194d7-a688-468e-b7a3-6ae8cd5ec1e4".to_string());
        sensor.unit_id = Some("0508c778-e84e-4bc6-b143-da485bdb7682".to_string());
        sensor.clean_sensor_empty_unit();
        assert_eq!(
            sensor.unit,
            Some("286194d7-a688-468e-b7a3-6ae8cd5ec1e4".to_string())
        );
        assert_eq!(
            sensor.unit_id,
            Some("0508c778-e84e-4bc6-b143-da485bdb7682".to_string())
        );

        let mut sensor = ModbusTcpNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        sensor.clean_empty_id();
        assert_eq!(
            sensor.id,
            Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string())
        );

        sensor.id = Some("".to_string());
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        sensor.id = None;
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        sensor.unit = Some("".to_string());
        sensor.unit_id = Some("".to_string());
        sensor.clean_sensor_empty_unit();
        assert_eq!(sensor.unit, None);
        assert_eq!(sensor.unit_id, None);

        sensor.unit = Some("286194d7-a688-468e-b7a3-6ae8cd5ec1e4".to_string());
        sensor.unit_id = Some("0508c778-e84e-4bc6-b143-da485bdb7682".to_string());
        sensor.clean_sensor_empty_unit();
        assert_eq!(
            sensor.unit,
            Some("286194d7-a688-468e-b7a3-6ae8cd5ec1e4".to_string())
        );
        assert_eq!(
            sensor.unit_id,
            Some("0508c778-e84e-4bc6-b143-da485bdb7682".to_string())
        );

        let mut sensor = BacnetIpNonNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        sensor.clean_empty_id();
        assert_eq!(
            sensor.id,
            Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string())
        );

        sensor.id = Some("".to_string());
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        sensor.id = None;
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        let mut sensor = ModbusTcpNonNumericSensor {
            id: Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string()),
            ..Default::default()
        };
        sensor.clean_empty_id();
        assert_eq!(
            sensor.id,
            Some("13d2cbd0-77c0-49a4-b9c8-38d91ce957d8".to_string())
        );

        sensor.id = Some("".to_string());
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);

        sensor.id = None;
        sensor.clean_empty_id();
        assert_eq!(sensor.id, None);
    }
}
