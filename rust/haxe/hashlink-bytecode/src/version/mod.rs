//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Version {
    V1,
    V2,
    V3,
    V4,
    V5,
}

impl Version {
    pub fn from_raw(raw: u8) -> Option<Version> {
        match raw {
            1 => Some(Version::V1),
            2 => Some(Version::V2),
            3 => Some(Version::V3),
            4 => Some(Version::V4),
            5 => Some(Version::V5),
            _ => None,
        }
    }

    pub fn needs_skip_assigns(&self) -> bool {
        match self {
            Version::V1 => false,
            Version::V2 => false,
            Version::V3 => true,
            Version::V4 => true,
            Version::V5 => true,
        }
    }

    pub fn is_supported(&self) -> bool {
        match self {
            Version::V1 => false,
            Version::V2 => false,
            Version::V3 => false,
            Version::V4 => true,
            Version::V5 => true,
        }
    }

    pub fn has_bytes_table(&self) -> bool {
        match self {
            Version::V1 => false,
            Version::V2 => false,
            Version::V3 => false,
            Version::V4 => false,
            Version::V5 => true,
        }
    }

    pub fn has_constants_table(&self) -> bool {
        match self {
            Version::V1 => false,
            Version::V2 => false,
            Version::V3 => false,
            Version::V4 => true,
            Version::V5 => true,
        }
    }
}
