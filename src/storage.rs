use crate::common::*;
pub trait Hardware {
    fn get_heath(&self) -> String;
    fn get_odata_context(&self) -> String;
    fn get_odata_id(&self) -> String;
    fn get_odata_type(&self) -> String;
    fn get_description(&self) -> String;
    fn get_firmware_version(&self) -> Firmware;
    fn get_id(&self) -> String;
    fn get_location(&self) -> String;
    fn get_location_format(&self) -> String;
    fn get_model(&self) -> String;
    fn get_name(&self) -> String;
    fn get_serial_number(&self) -> String;
    fn get_status(&self) -> AllStatus;
    fn get_state(&self) -> String;
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct HardwareCommon {
    #[serde(flatten)]
    pub odata: OData,
    pub description: String,
    pub id: String,
    pub firmware_version: Firmware,
    pub location: String,
    pub location_format: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub status: AllStatus,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct ArrayController {
    pub adapter_type: String,
    pub backup_power_source_status: String,
    pub current_operating_mode: String,
    pub encryption_crypto_officer_password_set: bool,
    pub encryption_enabled: bool,
    pub encryption_fw_locked: bool,
    pub encryption_has_locked_volumes_missing_boot_password: bool,
    pub encryption_mixed_volumes_enabled: bool,
    pub encryption_standalone_mode_enabled: bool,
    pub external_port_count: i64,
    #[serde(flatten)]
    pub hardware_common: HardwareCommon,
    pub hardware_revision: String,
    pub internal_port_count: i64,

    #[serde(rename = "Type")]
    pub controller_type: String,
    #[serde(rename = "links")]
    pub links: serde_json::Value,
}

impl Hardware for ArrayController {
    fn get_heath(&self) -> String {
        self.hardware_common.status.health.to_owned()
    }
    fn get_odata_context(&self) -> String {
        self.hardware_common.odata.odata_context.to_owned()
    }
    fn get_odata_id(&self) -> String {
        self.hardware_common.odata.odata_id.to_owned()
    }
    fn get_odata_type(&self) -> String {
        self.hardware_common.odata.odata_type.to_owned()
    }
    fn get_description(&self) -> String {
        self.hardware_common.description.to_owned()
    }
    fn get_firmware_version(&self) -> Firmware {
        self.hardware_common.firmware_version.to_owned()
    }
    fn get_id(&self) -> String {
        self.hardware_common.id.to_owned()
    }
    fn get_location(&self) -> String {
        self.hardware_common.location.to_owned()
    }
    fn get_location_format(&self) -> String {
        self.hardware_common.location_format.to_owned()
    }
    fn get_model(&self) -> String {
        self.hardware_common.model.to_owned()
    }
    fn get_name(&self) -> String {
        self.hardware_common.name.to_owned()
    }
    fn get_serial_number(&self) -> String {
        self.hardware_common.serial_number.to_owned()
    }
    fn get_status(&self) -> AllStatus {
        self.hardware_common.status.to_owned()
    }
    fn get_state(&self) -> String {
        self.hardware_common.status.state.to_owned()
    }
}

#[test]
fn test_array_controller_parser() {
    let test_data = include_str!("../tests/array-controller.json");
    let result: ArrayController = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct MultHardware {
    #[serde(flatten)]
    pub odata: OData,
    pub description: String,
    pub member_type: String,
    pub members: Vec<ODataId>,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    pub name: String,
    pub total: i64,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct ArrayControllers {
    #[serde(flatten)]
    pub mult_hardware: MultHardware,
    #[serde(rename = "Type")]
    pub controller_type: String,
    #[serde(rename = "links")]
    pub links: serde_json::Value,
}

#[test]
fn test_array_controllers_parser() {
    let test_data = include_str!("../tests/array-controllers.json");
    let result: ArrayControllers = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub logical_drives: Href,
    pub physical_drives: Href,
    pub storage_enclosures: Href,
    pub unconfigured_drives: Href,
    #[serde(flatten)]
    pub self_url: SelfLink,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct SmartArray {
    pub adapter_type: String,
    pub backup_power_source_status: String,
    pub current_operating_mode: String,
    pub encryption_crypto_officer_password_set: bool,
    pub encryption_enabled: bool,
    pub encryption_fw_locked: bool,
    pub encryption_has_locked_volumes_missing_boot_password: bool,
    pub encryption_mixed_volumes_enabled: bool,
    pub encryption_standalone_mode_enabled: bool,
    pub external_port_count: i64,
    pub hardware_revision: String,
    #[serde(flatten)]
    pub hardware_common: HardwareCommon,
    pub internal_port_count: i64,
    #[serde(rename = "Type")]
    pub array_type: String,
    #[serde(rename = "links")]
    pub links: Link,
}

impl Hardware for SmartArray {
    fn get_heath(&self) -> String {
        self.hardware_common.status.health.to_owned()
    }
    fn get_odata_context(&self) -> String {
        self.hardware_common.odata.odata_context.to_owned()
    }
    fn get_odata_id(&self) -> String {
        self.hardware_common.odata.odata_id.to_owned()
    }
    fn get_odata_type(&self) -> String {
        self.hardware_common.odata.odata_type.to_owned()
    }
    fn get_description(&self) -> String {
        self.hardware_common.description.to_owned()
    }
    fn get_firmware_version(&self) -> Firmware {
        self.hardware_common.firmware_version.to_owned()
    }
    fn get_id(&self) -> String {
        self.hardware_common.id.to_owned()
    }
    fn get_location(&self) -> String {
        self.hardware_common.location.to_owned()
    }
    fn get_location_format(&self) -> String {
        self.hardware_common.location_format.to_owned()
    }
    fn get_model(&self) -> String {
        self.hardware_common.model.to_owned()
    }
    fn get_name(&self) -> String {
        self.hardware_common.name.to_owned()
    }
    fn get_serial_number(&self) -> String {
        self.hardware_common.serial_number.to_owned()
    }
    fn get_status(&self) -> AllStatus {
        self.hardware_common.status.to_owned()
    }
    fn get_state(&self) -> String {
        self.hardware_common.status.state.to_owned()
    }
}

#[test]
fn test_smart_array_parser() {
    let test_data = include_str!("../tests/smart-array.json");
    let result: SmartArray = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct StorageEnclosure {
    pub drive_bay_count: i64,
    #[serde(flatten)]
    pub hardware_common: HardwareCommon,
    #[serde(rename = "Type")]
    pub enclosure_type: String,
    #[serde(rename = "links")]
    pub links: serde_json::Value,
}

impl Hardware for StorageEnclosure {
    fn get_heath(&self) -> String {
        self.hardware_common.status.health.to_owned()
    }
    fn get_odata_context(&self) -> String {
        self.hardware_common.odata.odata_context.to_owned()
    }
    fn get_odata_id(&self) -> String {
        self.hardware_common.odata.odata_id.to_owned()
    }
    fn get_odata_type(&self) -> String {
        self.hardware_common.odata.odata_type.to_owned()
    }
    fn get_description(&self) -> String {
        self.hardware_common.description.to_owned()
    }
    fn get_firmware_version(&self) -> Firmware {
        self.hardware_common.firmware_version.to_owned()
    }
    fn get_id(&self) -> String {
        self.hardware_common.id.to_owned()
    }
    fn get_location(&self) -> String {
        self.hardware_common.location.to_owned()
    }
    fn get_location_format(&self) -> String {
        self.hardware_common.location_format.to_owned()
    }
    fn get_model(&self) -> String {
        self.hardware_common.model.to_owned()
    }
    fn get_name(&self) -> String {
        self.hardware_common.name.to_owned()
    }
    fn get_serial_number(&self) -> String {
        self.hardware_common.serial_number.to_owned()
    }
    fn get_status(&self) -> AllStatus {
        self.hardware_common.status.to_owned()
    }
    fn get_state(&self) -> String {
        self.hardware_common.status.state.to_owned()
    }
}

#[test]
fn test_storage_enclosure_parser() {
    let test_data = include_str!("../tests/storage-enclosure.json");
    let result: StorageEnclosure = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct EnclosuresLinks {
    pub member: Vec<Href>,
    #[serde(flatten)]
    pub self_url: SelfLink,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct StorageEnclosures {
    #[serde(flatten)]
    pub mult_hardware: MultHardware,
    #[serde(rename = "Type")]
    pub enclosure_type: String,
    #[serde(rename = "links")]
    pub links: EnclosuresLinks,
}

#[test]
fn test_storage_enclosures_parser() {
    let test_data = include_str!("../tests/storage-enclosures.json");
    let result: StorageEnclosures = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct DiskDrive {
    pub block_size_bytes: i64,
    #[serde(rename = "CapacityGB")]
    pub capacity_gb: i64,
    pub capacity_logical_blocks: i64,
    pub capacity_mi_b: i64,
    pub carrier_application_version: String,
    pub carrier_authentication_status: String,
    pub current_temperature_celsius: i64,
    pub disk_drive_status_reasons: Vec<String>,
    pub encrypted_drive: bool,
    #[serde(flatten)]
    pub hardware_common: HardwareCommon,
    pub interface_speed_mbps: i64,
    pub interface_type: String,
    pub maximum_temperature_celsius: i64,
    pub media_type: String,
    pub power_on_hours: Option<i64>,
    pub rotational_speed_rpm: i64,
    pub ssd_endurance_utilization_percentage: Option<f64>,
    #[serde(rename = "Type")]
    pub drive_type: String,
    #[serde(rename = "links")]
    pub links: SelfLink,
}

impl Hardware for DiskDrive {
    fn get_heath(&self) -> String {
        self.hardware_common.status.health.to_owned()
    }
    fn get_odata_context(&self) -> String {
        self.hardware_common.odata.odata_context.to_owned()
    }
    fn get_odata_id(&self) -> String {
        self.hardware_common.odata.odata_id.to_owned()
    }
    fn get_odata_type(&self) -> String {
        self.hardware_common.odata.odata_type.to_owned()
    }
    fn get_description(&self) -> String {
        self.hardware_common.description.to_owned()
    }
    fn get_firmware_version(&self) -> Firmware {
        self.hardware_common.firmware_version.to_owned()
    }
    fn get_id(&self) -> String {
        self.hardware_common.id.to_owned()
    }
    fn get_location(&self) -> String {
        self.hardware_common.location.to_owned()
    }
    fn get_location_format(&self) -> String {
        self.hardware_common.location_format.to_owned()
    }
    fn get_model(&self) -> String {
        self.hardware_common.model.to_owned()
    }
    fn get_name(&self) -> String {
        self.hardware_common.name.to_owned()
    }
    fn get_serial_number(&self) -> String {
        self.hardware_common.serial_number.to_owned()
    }
    fn get_status(&self) -> AllStatus {
        self.hardware_common.status.to_owned()
    }
    fn get_state(&self) -> String {
        self.hardware_common.status.state.to_owned()
    }
}

#[test]
fn test_storage_drive_parser() {
    let test_data = include_str!("../tests/disk-drive.json");
    let result: DiskDrive = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct JsonLink {
    pub member: Vec<Href>,
    #[serde(flatten)]
    pub self_url: SelfLink,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct DiskDrives {
    #[serde(flatten)]
    pub mult_hardware: MultHardware,
    #[serde(rename = "Type")]
    pub drive_type: String,
    #[serde(rename = "links")]
    pub links: JsonLink,
}

#[test]
fn test_storage_drives_parser() {
    let test_data = include_str!("../tests/disk-drives.json");
    let result: DiskDrives = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct LogicalDrives {
    #[serde(flatten)]
    pub odata: OData,
    pub description: String,
    pub member_type: String,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    pub name: String,
    pub total: i64,
    #[serde(rename = "Type")]
    pub drive_type: String,
    #[serde(rename = "links")]
    pub links: SelfLink,
}

#[test]
fn test_storage_logical_drives_parser() {
    let test_data = include_str!("../tests/logical-drives.json");
    let result: LogicalDrives = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}
