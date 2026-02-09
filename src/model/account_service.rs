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
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::OData;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerAccount {
    #[serde(flatten)]
    pub odata: OData,

    // Set on GET. Do not set for POST/PATCH
    pub id: Option<String>,

    #[serde(rename = "UserName")]
    pub username: String,

    // Set this for POST/PATCH. Not populated by GET.
    pub password: Option<String>,

    // A RoleId converted to string
    pub role_id: String,

    pub name: Option<String>,
    pub description: Option<String>,
    pub enabled: Option<bool>,
    pub locked: Option<bool>,
}

impl Ord for ManagerAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for ManagerAccount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ManagerAccount {
    fn eq(&self, other: &ManagerAccount) -> bool {
        self.id == other.id
    }
}
