# libredfish

[![Crates.io](https://img.shields.io/crates/v/libredfish.svg)](https://crates.io/crates/libredfish)
[![docs.rs](https://docs.rs/libredfish/badge.svg)](https://docs.rs/libredfish)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A native async Rust client for [Redfish](https://www.dmtf.org/standards/redfish), the DMTF standard
API for managing server hardware through a BMC (Baseboard Management Controller).

`libredfish` gives you a single, vendor-agnostic `Redfish` trait for common operations — power
control, BIOS/boot management, firmware updates, sensors, event logs, secure boot, user
management, and more — while handling the quirks of each vendor's Redfish implementation under
the hood.

## Supported vendors

- Dell (iDRAC)
- HPE (iLO)
- Lenovo
- Supermicro
- AMI (Viking DGX H100)
- NVIDIA BlueField DPU
- NVIDIA GH200 (Grace Hopper)
- NVIDIA GBx00 (Grace-Blackwell)
- NVIDIA GB NVLink switch
- LiteOn Power Shelf

The client detects the vendor automatically from the BMC's service root and dispatches calls to
the matching implementation, so application code can be written once against the `Redfish` trait.

## Installation

```toml
[dependencies]
libredfish = "0.2"
```

## Usage

```rust
use libredfish::{Endpoint, RedfishClientPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Endpoint {
        host: "bmc.example.com".to_string(),
        user: Some("admin".to_string()),
        password: Some("password".to_string()),
        ..Default::default()
    };

    let pool = RedfishClientPool::builder().build()?;

    // Detects the BMC vendor and returns a client implementing the `Redfish` trait.
    let redfish = pool.create_client(endpoint).await?;

    let power_state = redfish.get_power_state().await?;
    println!("Power state: {power_state:?}");

    let system = redfish.get_system().await?;
    println!("System: {system:?}");

    Ok(())
}
```

If you already know the vendor speaks the standard Redfish schema (no vendor-specific quirks
needed), you can skip auto-detection with `create_standard_client`:

```rust
let redfish = pool.create_standard_client(endpoint)?;
```

## Development

Integration tests run against the [DMTF Redfish mockup server](https://github.com/DMTF/Redfish-Mockup-Server)
included under `tests/`:

```bash
./tests/run_mock_server.sh
cargo test
```

## License

Licensed under the [MIT license](LICENSE).
