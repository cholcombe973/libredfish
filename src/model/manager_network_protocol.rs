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

use crate::model::ODataLinks;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Protocol {
    pub port: Option<i64>,
    pub protocol_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerNetworkProtocol {
    #[serde(flatten)]
    pub odata: ODataLinks,
    pub name: Option<String>,
    #[serde(rename = "DHCP")]
    pub dhcp: Option<Protocol>,
    #[serde(rename = "DHCPv6")]
    pub dhcpv6: Option<Protocol>,
    pub description: Option<String>,
    #[serde(rename = "FQDN")]
    pub fqdn: Option<String>,
    #[serde(rename = "HTTP")]
    pub http: Option<Protocol>,
    pub host_name: Option<String>,
    #[serde(rename = "IPMI")]
    pub ipmi: Option<Protocol>,
    pub id: Option<String>,
    #[serde(rename = "KVMIP")]
    pub kvmip: Option<Protocol>,
    pub rdp: Option<Protocol>,
    #[serde(rename = "RFB")]
    pub rfb: Option<Protocol>,
    pub ssh: Option<Protocol>,
    #[serde(rename = "SNMP")]
    pub snmp: Option<Protocol>,
    pub telnet: Option<Protocol>,
    pub virtual_media: Option<Protocol>,
}
