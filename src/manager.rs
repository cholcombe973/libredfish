use crate::common::*;

#[derive(Debug, Deserialize)]
pub struct ActionsManagerReset {
    pub target: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Action {
    #[serde(rename = "#Manager.Reset")]
    pub manager_reset: ActionsManagerReset,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Availableaction {
    pub action: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Commandshell {
    pub connect_types_supported: Vec<String>,
    pub enabled: bool,
    pub max_concurrent_sessions: i64,
    pub service_enabled: bool,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Ethernetinterface {
    #[serde(flatten)]
    pub odata_id: ODataId,
}

#[derive(Debug, Deserialize)]
pub struct OemHpActionshpiloResetToFactoryDefault {
    #[serde(rename = "ResetType@Redfish.AllowableValues")]
    pub reset_type_redfish_allowable_values: Vec<String>,
    pub target: String,
}

#[derive(Debug, Deserialize)]
pub struct OemHpAction {
    #[serde(rename = "#HpiLO.ClearRestApiState")]
    pub hpi_lo_clear_rest_api_state: ActionsManagerReset,
    #[serde(rename = "#HpiLO.ResetToFactoryDefaults")]
    pub hpi_lo_reset_to_factory_defaults: OemHpActionshpiloResetToFactoryDefault,
    #[serde(rename = "#HpiLO.iLOFunctionality")]
    pub hpi_lo_i_lo_functionality: ActionsManagerReset,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpAvailableactionsCapability {
    pub allowable_values: Vec<String>,
    pub property_name: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpAvailableaction {
    pub action: String,
    pub capabilities: Vec<OemHpAvailableactionsCapability>,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpFederationconfig {
    #[serde(rename = "IPv6MulticastScope")]
    pub i_pv6_multicast_scope: String,
    pub multicast_announcement_interval: i64,
    pub multicast_discovery: String,
    pub multicast_time_to_live: i64,
    #[serde(rename = "iLOFederationManagement")]
    pub i_lo_federation_management: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpFirmwareCurrent {
    pub date: String,
    pub debug_build: bool,
    pub major_version: i64,
    pub minor_version: i64,
    pub time: String,
    pub version_string: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpFirmware {
    pub current: OemHpFirmwareCurrent,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpLicense {
    pub license_key: String,
    pub license_string: String,
    pub license_type: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpIloselftestresult {
    pub notes: String,
    pub self_test_name: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtRef {
    pub extref: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHpLink {
    pub active_health_system: Href,
    pub date_time_service: Href,
    pub embedded_media_service: Href,
    pub federation_dispatch: ExtRef,
    pub federation_groups: Href,
    pub federation_peers: Href,
    pub license_service: Href,
    pub security_service: Href,
    pub update_service: Href,
    #[serde(rename = "VSPLogLocation")]
    pub vsp_log_location: ExtRef,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct OemHp {
    #[serde(flatten)]
    pub odata_type: ODataType,
    pub actions: OemHpAction,
    pub available_actions: Vec<OemHpAvailableaction>,
    pub clear_rest_api_status: String,
    pub federation_config: OemHpFederationconfig,
    pub firmware: OemHpFirmware,
    pub license: OemHpLicense,
    #[serde(rename = "RequiredLoginForiLORBSU")]
    pub required_login_fori_lorbsu: bool,
    #[serde(rename = "SerialCLISpeed")]
    pub serial_cli_speed: i64,
    #[serde(rename = "SerialCLIStatus")]
    pub serial_cli_status: String,
    #[serde(rename = "Type")]
    pub oem_type: String,
    #[serde(rename = "VSPLogDownloadEnabled")]
    pub vsp_log_download_enabled: bool,
    #[serde(rename = "iLOSelfTestResults")]
    pub i_lo_self_test_results: Vec<OemHpIloselftestresult>,
    #[serde(rename = "links")]
    pub links: OemHpLink,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Oem {
    pub hp: OemHp,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Status {
    pub state: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "EthernetNICs")]
    pub ethernet_nics: Href,
    pub logs: Href,
    pub manager_for_chassis: Vec<Href>,
    pub manager_for_servers: Vec<Href>,
    pub network_service: Href,
    pub virtual_media: Href,
    #[serde(flatten)]
    pub self_url: crate::common::Link,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize)]
pub struct Manager {
    #[serde(flatten)]
    pub odata: OData,
    pub actions: Action,
    pub available_actions: Vec<Availableaction>,
    pub command_shell: Commandshell,
    pub description: String,
    pub ethernet_interfaces: Ethernetinterface,
    pub firmware: Firmware,
    pub firmware_version: String,
    pub graphical_console: Commandshell,
    pub id: String,
    pub log_services: Ethernetinterface,
    pub manager_type: String,
    pub name: String,
    pub network_protocol: Ethernetinterface,
    pub oem: Oem,
    pub serial_console: Commandshell,
    pub status: Status,
    #[serde(rename = "Type")]
    pub root_type: String,
    #[serde(rename = "UUID")]
    pub uuid: String,
    pub virtual_media: Ethernetinterface,
    #[serde(rename = "links")]
    pub links: Link,
}

#[test]
fn test_manager_parser() {
    let test_data = include_str!("../tests/manager.json");
    let result: Manager = serde_json::from_str(&test_data).unwrap();
    println!("result: {:#?}", result);
}
