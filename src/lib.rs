#[macro_use]
extern crate serde_derive;

pub mod manager;
pub mod power;
pub mod storage;
pub mod thermal;

use reqwest::{header::HeaderValue, header::ACCEPT, header::CONTENT_TYPE, Client};
use serde::de::DeserializeOwned;

pub struct Config {
    pub user: Option<String>,
    pub endpoint: String,
    pub password: Option<String>,
    pub port: Option<u16>,
}

fn get<T>(client: &reqwest::Client, config: &Config, api: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned + ::std::fmt::Debug,
{
    let url = match config.port {
        Some(p) => format!("https://{}:{}/{}", config.endpoint, p, api),
        None => format!("https://{}/{}", config.endpoint, api),
    };

    let res: T = match &config.user {
        Some(user) => client
            .get(&url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .basic_auth(&user, config.password.as_ref())
            .send()?
            .error_for_status()?
            .json()?,
        None => client
            .get(&url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .send()?
            .error_for_status()?
            .json()?,
    };
    Ok(res)
}

pub fn get_array_controller(
    c: &Client,
    config: &Config,
    controller_id: u64,
) -> Result<storage::ArrayController, reqwest::Error> {
    let url = format!("Systems/1/SmartStorage/ArrayControllers/{}/", controller_id);
    let s: storage::ArrayController = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_array_controllers(
    c: &Client,
    config: &Config,
) -> Result<storage::ArrayControllers, reqwest::Error> {
    let url = "Systems/1/SmartStorage/ArrayControllers/";
    let s: storage::ArrayControllers = get(c, &config, &url)?;
    Ok(s)
}

/// Query the manager status from the server
pub fn get_manager_status(c: &Client, config: &Config) -> Result<manager::Manager, reqwest::Error> {
    let url = "Managers/";
    let m: manager::Manager = get(c, &config, &url)?;
    Ok(m)
}

/// Query the power status from the server
pub fn get_power_status(c: &Client, config: &Config) -> Result<power::Power, reqwest::Error> {
    let url = "Chassis/1/Power/";
    let p: power::Power = get(c, &config, &url)?;
    Ok(p)
}

/// Query the thermal status from the server
pub fn get_thermal_status(c: &Client, config: &Config) -> Result<thermal::Thermal, reqwest::Error> {
    let url = "Chassis/1/Thermal/";
    let t: thermal::Thermal = get(c, &config, &url)?;
    Ok(t)
}

/// Query the smart array status from the server
pub fn get_smart_array_status(
    c: &Client,
    config: &Config,
    controller_id: u64,
) -> Result<storage::SmartArray, reqwest::Error> {
    let url = format!("Systems/1/SmartStorage/ArrayControllers/{}/", controller_id);
    let s: storage::SmartArray = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_logical_drives(
    c: &Client,
    config: &Config,
    controller_id: u64,
) -> Result<storage::LogicalDrives, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/{}/LogicalDrives/",
        controller_id
    );
    let s: storage::LogicalDrives = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_physical_drive(
    c: &Client,
    config: &Config,
    drive_id: u64,
    controller_id: u64,
) -> Result<storage::DiskDrive, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/{}/DiskDrives/{}/",
        controller_id, drive_id,
    );
    let d: storage::DiskDrive = get(c, &config, &url)?;
    Ok(d)
}

pub fn get_physical_drives(
    c: &Client,
    config: &Config,
    controller_id: u64,
) -> Result<storage::DiskDrives, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/{}/DiskDrives/",
        controller_id
    );
    let d: storage::DiskDrives = get(c, &config, &url)?;
    Ok(d)
}

pub fn get_storage_enclosures(
    c: &Client,
    config: &Config,
    controller_id: u64,
) -> Result<storage::StorageEnclosures, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/{}/StorageEnclosures/",
        controller_id
    );
    let s: storage::StorageEnclosures = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_storage_enclosure(
    c: &Client,
    config: &Config,
    controller_id: u64,
    enclosure_id: u64,
) -> Result<storage::StorageEnclosure, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/{}/StorageEnclosures/{}/",
        controller_id, enclosure_id,
    );
    let s: storage::StorageEnclosure = get(c, &config, &url)?;
    Ok(s)
}
