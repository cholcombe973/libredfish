use crate::common::*;
#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct FansOemHp {
    #[serde(flatten)]
    pub odata_type: ODataType,
    pub location: String,
    #[serde(rename = "Type")]
    pub fan_type: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct FansOem {
    pub hp: FansOemHp,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Fan {
    pub current_reading: i64,
    pub fan_name: String,
    pub oem: FansOem,
    pub status: SomeStatus,
    pub units: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct TemperaturesOemHp {
    #[serde(flatten)]
    pub odata_type: ODataType,
    pub location_xmm: i64,
    pub location_ymm: i64,
    #[serde(rename = "Type")]
    pub temp_type: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct TemperaturesOem {
    pub hp: TemperaturesOemHp,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Temperature {
    pub current_reading: i64,
    pub name: String,
    pub number: i64,
    pub lower_threshold_critical: Option<i64>,
    pub lower_threshold_fatal: Option<i64>,
    pub oem: TemperaturesOem,
    pub physical_context: String,
    pub reading_celsius: i64,
    pub status: SomeStatus,
    pub units: String,
    pub upper_threshold_critical: i64,
    pub upper_threshold_fatal: i64,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Thermal {
    #[serde(flatten)]
    pub odata: OData,
    pub fans: Vec<Fan>,
    pub id: String,
    pub name: String,
    pub temperatures: Vec<Temperature>,
    #[serde(rename = "Type")]
    pub thermal_type: String,
    #[serde(rename = "links")]
    pub links: Link,
}

#[test]
fn test_thermal_parser() {
    let test_data = include_str!("../tests/chassis-thermal.json");
    let result: Thermal = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}
