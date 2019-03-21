#[derive(Debug, Deserialize, Clone)]
pub struct FirmwareCurrent {
    #[serde(rename = "VersionString")]
    pub version: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct Firmware {
    pub current: FirmwareCurrent,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Href {
    pub href: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct OData {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ODataType {
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ODataLinks {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ODataId {
    #[serde(rename = "@odata.id")]
    pub odata_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ODataContext {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "links")]
    pub links: SelfLink,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct AllStatus {
    pub health: String,
    pub state: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct SomeStatus {
    pub health: Option<String>,
    pub state: String,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct SelfLink {
    #[serde(rename = "self")]
    pub self_url: Href,
}

#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, Clone)]
pub struct HpType {
    #[serde(rename = "@odata.type")]
    pub odata_type: String,
    #[serde(rename = "Type")]
    pub hp_type: String,
}
