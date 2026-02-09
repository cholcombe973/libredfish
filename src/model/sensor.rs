/*
 * SPDX-FileCopyrightText: Copyright (c) 2025 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
 * SPDX-License-Identifier: MIT
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */
use crate::model::{ODataId, ResourceStatus};
use crate::OData;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct GPUSensors {
    pub gpu_id: String,
    pub sensors: Vec<Sensor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Sensor {
    #[serde(flatten)]
    pub odata: OData,
    pub id: Option<String>,
    pub name: Option<String>,
    pub physical_context: Option<PhysicalContext>,
    pub reading: Option<f64>,
    pub reading_type: Option<ReadingType>,
    pub reading_units: Option<String>,
    pub reading_range_max: Option<f64>,
    pub reading_range_min: Option<f64>,
    pub status: Option<ResourceStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Sensors {
    #[serde(flatten)]
    pub odata: OData,
    pub members: Vec<ODataId>,
    pub name: String,
    pub description: Option<String>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq)]
pub enum PhysicalContext {
    #[default]
    Room,
    Intake,
    Exhaust,
    LiquidInlet,
    LiquidOutlet,
    Front,
    Back,
    Upper,
    Lower,
    CPU,
    CPUSubsystem,
    GPU,
    GPUSubsystem,
    FPGA,
    Accelerator,
    ASIC,
    Backplane,
    SystemBoard,
    PowerSupply,
    PowerSubsystem,
    VoltageRegulator,
    Rectifier,
    StorageDevice,
    NetworkingDevice,
    ComputeBay,
    StorageBay,
    NetworkBay,
    ExpansionBay,
    PowerSupplyBay,
    Memory,
    MemorySubsystem,
    Chassis,
    Fan,
    CoolingSubsystem,
    Motor,
    Transformer,
    ACUtilityInput,
    ACStaticBypassInput,
    ACMaintenanceBypassInput,
    DCBus,
    ACOutput,
    ACInput,
    TrustedModule,
    Board,
    Transceiver,
    Battery,
    Pump,
}

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone, Eq, PartialEq)]
pub enum ReadingType {
    #[default]
    Temperature,
    Humidity,
    Power,
    EnergykWh,
    EnergyJoules,
    EnergyWh,
    ChargeAh,
    Voltage,
    Current,
    Frequency,
    Pressure,
    PressurekPa,
    PressurePa,
    LiquidLevel,
    Rotational,
    AirFlow,
    AirFlowCMM,
    LiquidFlow,
    LiquidFlowLPM,
    Barometric,
    Altitude,
    Percent,
    AbsoluteHumidity,
    Heat,
}

impl Display for PhysicalContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
