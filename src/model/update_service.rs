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
use serde::{Deserialize, Serialize};

/// https://redfish.dmtf.org/schemas/v1/UpdateService.v1_14_0.json
/// Service for Software Update
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", default)]
pub struct UpdateService {
    pub http_push_uri: String,
    pub max_image_size_bytes: i32,
    pub multipart_http_push_uri: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferProtocolType {
    FTP,
    SFTP,
    HTTP,
    HTTPS,
    SCP,
    TFTP,
    OEM,
    NFS,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, clap::ValueEnum, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ComponentType {
    BMC,
    UEFI,
    EROTBMC,
    EROTBIOS,
    CPLDMID,
    CPLDMB,
    CPLDPDB,
    #[clap(skip)]
    PSU {
        num: u32,
    },
    #[clap(skip)]
    PCIeSwitch {
        num: u32,
    },
    #[clap(skip)]
    PCIeRetimer {
        num: u32,
    },
    HGXBMC,
    #[clap(skip)]
    Unknown,
}
