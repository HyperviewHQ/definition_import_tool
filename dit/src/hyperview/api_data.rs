use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetDefinition {
    pub id: String,
    pub name: String,
    #[serde(alias = "assetType")]
    pub asset_type: String,
    #[serde(alias = "associatedAssets")]
    pub associated_assets: usize,
}

impl fmt::Display for BacnetDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nasset type: {}\nassociated_assets: {}",
            self.id, self.name, self.asset_type, self.associated_assets
        )
    }
}

#[serde_as]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetIpNumericSensor {
    pub id: String,
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
    unit: String,
    #[serde(alias = "unitId")]
    #[serde_as(deserialize_as = "DefaultOnError")]
    unit_id: String,
}

impl fmt::Display for BacnetIpNumericSensor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nmultiplier: {}\nobject_instance: {}\nobject_type: {}\nsensor type: {}\nsensor type id: {}\nunit: {}\nunit id: {}",
            self.id, self.name, self.multiplier, self.object_instance, self.object_type, self.sensor_type, self.sensor_type_id, self.unit, self.unit_id
        )
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

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BacnetIpNonNumericSensor {
    id: String,
    name: String,
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
        let sensor_header = format!(
            "id: {}\nname: {}\nobject type: {}\nsensor type: {}\nsensor type id: {}",
            self.id, self.name, self.object_type, self.sensor_type, self.sensor_type_id
        );
        let sensor_value_mapping = &self
            .value_mapping
            .iter()
            .fold(String::new(), |acc, m| acc + "\n" + &m.to_string());

        write!(f, "{}\n{}", sensor_header, sensor_value_mapping)
    }
}

impl Serialize for BacnetIpNonNumericSensor {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BacnetIpNonNumericSensor", 7)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("objectType", &self.object_type)?;
        state.serialize_field("sensorType", &self.sensor_type)?;
        state.serialize_field("sensorTypeId", &self.sensor_type_id)?;

        let value_mapping_str = self
            .value_mapping
            .iter()
            .map(|vm| format!("{}:{}", vm.text, vm.value))
            .collect::<Vec<String>>()
            .join(",");

        state.serialize_field("valueMapping", &value_mapping_str)?;

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_csv_serialization() {
        let sensor = BacnetIpNonNumericSensor {
            id: "247a4ad9-9d18-4bf4-b20b-a1d7d61b3971".to_string(),
            name: "Sensor 1".to_string(),
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
        };

        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&sensor).expect("Failed to serialize sensor");

        let data = String::from_utf8(wtr.into_inner().expect("Failed to get inner writer"))
            .expect("Failed to convert to string");

        let expected_data = "id,name,objectType,sensorType,sensorTypeId,valueMapping\n247a4ad9-9d18-4bf4-b20b-a1d7d61b3971,Sensor 1,Temperature,Analog,1000,\"Low:0,High:1\"\n";

        assert_eq!(data, expected_data);
    }
}
