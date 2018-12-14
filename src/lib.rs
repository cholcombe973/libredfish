#[macro_use]
extern crate serde_derive;

pub mod manager;
pub mod power;
pub mod storage;
pub mod thermal;

use reqwest::{header::HeaderValue, header::ACCEPT, header::CONTENT_TYPE, Client};
use serde::de::DeserializeOwned;

pub struct Config {
    user: String,
    password: String,
    endpoint: String,
}

pub fn get<T>(client: &reqwest::Client, config: &Config, url: &str) -> Result<T, reqwest::Error>
where
    T: DeserializeOwned,
{
    let url = format!("https://{}/redfish/v1/{}", config.endpoint, url);
    let j: T = client
        .get(&url)
        .basic_auth(&config.user, Some(&config.password))
        .header(ACCEPT, HeaderValue::from_static("application/json"))
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .send()?
        .error_for_status()?
        .json()?;
    Ok(j)
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
) -> Result<storage::SmartArray, reqwest::Error> {
    let url = "Systems/1/SmartStorage/ArrayControllers/1/";
    let s: storage::SmartArray = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_logical_drives(
    c: &Client,
    config: &Config,
) -> Result<storage::LogicalDrives, reqwest::Error> {
    let url = "Systems/1/SmartStorage/ArrayControllers/1/LogicalDrives/";
    let s: storage::LogicalDrives = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_physical_drive(
    c: &Client,
    config: &Config,
    drive_id: u64,
) -> Result<storage::DiskDrive, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/1/DiskDrives/{}/",
        drive_id,
    );
    let d: storage::DiskDrive = get(c, &config, &url)?;
    Ok(d)
}

pub fn get_physical_drives(
    c: &Client,
    config: &Config,
) -> Result<storage::DiskDrives, reqwest::Error> {
    let url = "Systems/1/SmartStorage/ArrayControllers/1/DiskDrives/";
    let d: storage::DiskDrives = get(c, &config, &url)?;
    Ok(d)
}

pub fn get_storage_enclosures(
    c: &Client,
    config: &Config,
) -> Result<storage::StorageEnclosures, reqwest::Error> {
    let url = "Systems/1/SmartStorage/ArrayControllers/1/StorageEnclosures/";
    let s: storage::StorageEnclosures = get(c, &config, &url)?;
    Ok(s)
}

pub fn get_storage_enclosure(
    c: &Client,
    config: &Config,
    enclosure_id: u64,
) -> Result<storage::StorageEnclosure, reqwest::Error> {
    let url = format!(
        "Systems/1/SmartStorage/ArrayControllers/1/StorageEnclosures/{}/",
        enclosure_id,
    );
    let s: storage::StorageEnclosure = get(c, &config, &url)?;
    Ok(s)
}
