pub trait Hardware {
    fn get_heath(&self) -> String;
    fn get_odata_context(&self) -> String;
    fn get_odata_id(&self) -> String;
    fn get_odata_type(&self) -> String;
    fn get_description(&self) -> String;
    fn get_firmware_version(&self) -> FirmwareVersion;
    fn get_id(&self) -> String;
    fn get_location(&self) -> String;
    fn get_location_format(&self) -> String;
    fn get_model(&self) -> String;
    fn get_name(&self) -> String;
    fn get_serial_number(&self) -> String;
    fn get_status(&self) -> Status;
    fn get_state(&self) -> String;
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct ArrayController {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub adapter_type: String,
    pub backup_power_source_status: String,
    pub current_operating_mode: String,
    pub description: String,
    pub encryption_crypto_officer_password_set: bool,
    pub encryption_enabled: bool,
    pub encryption_fw_locked: bool,
    pub encryption_has_locked_volumes_missing_boot_password: bool,
    pub encryption_mixed_volumes_enabled: bool,
    pub encryption_standalone_mode_enabled: bool,
    pub external_port_count: i64,
    pub firmware_version: FirmwareVersion,
    pub hardware_revision: String,
    pub id: String,
    pub internal_port_count: i64,
    pub location: String,
    pub location_format: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub status: Status,
    #[serde(rename = "Type")]
    pub controller_type: String,
    #[serde(rename = "links")]
    pub links: serde_json::Value,
}

impl Hardware for ArrayController {
    fn get_heath(&self) -> String {
        self.status.health.to_owned()
    }
    fn get_odata_context(&self) -> String {
        self.odata_context.to_owned()
    }
    fn get_odata_id(&self) -> String {
        self.odata_id.to_owned()
    }
    fn get_odata_type(&self) -> String {
        self.odata_type.to_owned()
    }
    fn get_description(&self) -> String {
        self.description.to_owned()
    }
    fn get_firmware_version(&self) -> FirmwareVersion {
        self.firmware_version.clone()
    }
    fn get_id(&self) -> String {
        self.id.to_owned()
    }
    fn get_location(&self) -> String {
        self.location.to_owned()
    }
    fn get_location_format(&self) -> String {
        self.location_format.to_owned()
    }
    fn get_model(&self) -> String {
        self.model.to_owned()
    }
    fn get_name(&self) -> String {
        self.name.to_owned()
    }
    fn get_serial_number(&self) -> String {
        self.serial_number.to_owned()
    }
    fn get_status(&self) -> Status {
        self.status.to_owned()
    }
    fn get_state(&self) -> String {
        self.status.state.to_owned()
    }
}

#[test]
fn test_array_controller_parser() {
    let test_data = include_str!("../tests/array-controller.json");
    let result: ArrayController = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct ArrayControllers {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub description: String,
    pub member_type: String,
    pub members: Vec<Member>,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    pub name: String,
    pub total: i64,
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
#[derive(Debug, Deserialize)]
pub struct FirmwareVersionCurrent {
    pub version_string: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct FirmwareVersion {
    pub current: FirmwareVersionCurrent,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Status {
    pub health: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct Href {
    pub href: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Link {
    pub logical_drives: Href,
    pub physical_drives: Href,
    pub storage_enclosures: Href,
    pub unconfigured_drives: Href,
    #[serde(rename = "self")]
    pub self_url: Href,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct SmartArray {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub adapter_type: String,
    pub backup_power_source_status: String,
    pub current_operating_mode: String,
    pub description: String,
    pub encryption_crypto_officer_password_set: bool,
    pub encryption_enabled: bool,
    pub encryption_fw_locked: bool,
    pub encryption_has_locked_volumes_missing_boot_password: bool,
    pub encryption_mixed_volumes_enabled: bool,
    pub encryption_standalone_mode_enabled: bool,
    pub external_port_count: i64,
    pub firmware_version: FirmwareVersion,
    pub hardware_revision: String,
    pub id: String,
    pub internal_port_count: i64,
    pub location: String,
    pub location_format: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub status: Status,
    #[serde(rename = "Type")]
    pub array_type: String,
    #[serde(rename = "links")]
    pub links: Link,
}

#[test]
fn test_smart_array_parser() {
    let test_data = include_str!("../tests/smart-array.json");
    let result: SmartArray = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct StorageEnclosure {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub description: String,
    pub drive_bay_count: i64,
    pub firmware_version: FirmwareVersion,
    pub id: String,
    pub location: String,
    pub location_format: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub status: Status,
    #[serde(rename = "Type")]
    pub enclosure_type: String,
    #[serde(rename = "links")]
    pub links: serde_json::Value,
}

#[test]
fn test_storage_enclosure_parser() {
    let test_data = include_str!("../tests/storage-enclosure.json");
    let result: StorageEnclosure = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Member {
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct EnclosuresLinks {
    pub member: Vec<Href>,
    #[serde(rename = "self")]
    pub self_url: Href,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct StorageEnclosures {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub description: String,
    pub member_type: String,
    pub members: Vec<Member>,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    pub name: String,
    pub total: i64,
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

#[derive(Debug, Deserialize)]
pub struct DriveLinks {
    #[serde(rename = "self")]
    pub self_url: Href,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct DiskDrive {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub block_size_bytes: i64,
    #[serde(rename = "CapacityGB")]
    pub capacity_gb: i64,
    pub capacity_logical_blocks: i64,
    pub capacity_mi_b: i64,
    pub carrier_application_version: String,
    pub carrier_authentication_status: String,
    pub current_temperature_celsius: i64,
    pub description: String,
    pub disk_drive_status_reasons: Vec<String>,
    pub encrypted_drive: bool,
    pub firmware_version: FirmwareVersion,
    pub id: String,
    pub interface_speed_mbps: i64,
    pub interface_type: String,
    pub location: String,
    pub location_format: String,
    pub maximum_temperature_celsius: i64,
    pub media_type: String,
    pub model: String,
    pub name: String,
    pub power_on_hours: Option<i64>,
    pub rotational_speed_rpm: i64,
    pub ssd_endurance_utilization_percentage: Option<f64>,
    pub serial_number: String,
    pub status: Status,
    #[serde(rename = "Type")]
    pub drive_type: String,
    #[serde(rename = "links")]
    pub links: DriveLinks,
}

#[test]
fn test_storage_drive_parser() {
    let test_data = include_str!("../tests/disk-drive.json");
    let result: DiskDrive = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct JsonLink {
    pub member: Vec<Href>,
    #[serde(rename = "self")]
    pub self_url: Href,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct DiskDrives {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub description: String,
    pub member_type: String,
    pub members: Vec<Member>,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    pub name: String,
    pub total: i64,
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
#[derive(Debug, Deserialize)]
pub struct LogicalDrives {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    pub description: String,
    pub member_type: String,
    #[serde(rename = "Members@odata.count")]
    pub members_odata_count: i64,
    pub name: String,
    pub total: i64,
    #[serde(rename = "Type")]
    pub drive_type: String,
    #[serde(rename = "links")]
    pub links: DriveLinks,
}

#[test]
fn test_storage_logical_drives_parser() {
    let test_data = include_str!("../tests/logical-drives.json");
    let result: LogicalDrives = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}
