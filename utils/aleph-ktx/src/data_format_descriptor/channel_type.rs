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

///
/// Channel types for BC1 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC1ChannelType {
    Color,
    ColorAndAlpha,
}

impl BC1ChannelType {
    ///
    /// Converts the raw value to one of the possible BC1 channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC1ChannelType::Color),
            1 => Some(BC1ChannelType::ColorAndAlpha),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC1ChannelType::Color => 0,
            BC1ChannelType::ColorAndAlpha => 1,
        }
    }
}

///
/// Channel types for BC2 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC2ChannelType {
    Color,
    Alpha,
}

impl BC2ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC2ChannelType::Color),
            15 => Some(BC2ChannelType::Alpha),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC2ChannelType::Color => 0,
            BC2ChannelType::Alpha => 15,
        }
    }
}

///
/// Channel types for BC3 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC3ChannelType {
    Color,
    Alpha,
}

impl BC3ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC3ChannelType::Color),
            15 => Some(BC3ChannelType::Alpha),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC3ChannelType::Color => 0,
            BC3ChannelType::Alpha => 15,
        }
    }
}

///
/// Channel types for BC4 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC4ChannelType {
    Data,
}

impl BC4ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC4ChannelType::Data),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC4ChannelType::Data => 0,
        }
    }
}

///
/// Channel types for BC5 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC5ChannelType {
    Red,
    Green,
}

impl BC5ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC5ChannelType::Red),
            1 => Some(BC5ChannelType::Green),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC5ChannelType::Red => 0,
            BC5ChannelType::Green => 1,
        }
    }
}

///
/// Channel types for BC6 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC6ChannelType {
    Color,
}

impl BC6ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC6ChannelType::Color),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC6ChannelType::Color => 0,
        }
    }
}

///
/// Channel types for BC7 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum BC7ChannelType {
    Color,
}

impl BC7ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(BC7ChannelType::Color),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            BC7ChannelType::Color => 0,
        }
    }
}

///
/// Channel types for ETC1 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum ETC1ChannelType {
    Color,
}

impl ETC1ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(ETC1ChannelType::Color),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            ETC1ChannelType::Color => 0,
        }
    }
}

///
/// Channel types for ETC2 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum ETC2ChannelType {
    Red,
    Green,
    Color,
    Alpha,
}

impl ETC2ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(ETC2ChannelType::Red),
            1 => Some(ETC2ChannelType::Green),
            2 => Some(ETC2ChannelType::Color),
            15 => Some(ETC2ChannelType::Alpha),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            ETC2ChannelType::Red => 0,
            ETC2ChannelType::Green => 1,
            ETC2ChannelType::Color => 2,
            ETC2ChannelType::Alpha => 15,
        }
    }
}

///
/// Channel types for ASTC compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum ASTCChannelType {
    Data,
}

impl ASTCChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(ASTCChannelType::Data),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            ASTCChannelType::Data => 0,
        }
    }
}

///
/// Channel types for ETC1S compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum ETC1SChannelType {
    Color,
}

impl ETC1SChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(ETC1SChannelType::Color),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            ETC1SChannelType::Color => 0,
        }
    }
}

///
/// Channel types for PVRTC compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum PVRTCChannelType {
    Color,
}

impl PVRTCChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(PVRTCChannelType::Color),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            PVRTCChannelType::Color => 0,
        }
    }
}

///
/// Channel types for PVRTC2 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum PVRTC2ChannelType {
    Color,
}

impl PVRTC2ChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(PVRTC2ChannelType::Color),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            PVRTC2ChannelType::Color => 0,
        }
    }
}

///
/// Channel types for PVRTC2 compressed formats
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
pub enum RGBSDAChannelType {
    Red,
    Green,
    Blue,
    Stencil,
    Depth,
    Alpha,
}

impl RGBSDAChannelType {
    ///
    /// Converts the raw value to one of the channel types. Returns `None` if `val` is
    /// invalid
    ///
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            0 => Some(RGBSDAChannelType::Red),
            1 => Some(RGBSDAChannelType::Green),
            2 => Some(RGBSDAChannelType::Blue),
            13 => Some(RGBSDAChannelType::Stencil),
            14 => Some(RGBSDAChannelType::Depth),
            15 => Some(RGBSDAChannelType::Alpha),
            _ => None,
        }
    }

    ///
    /// Get the raw value for this channel type
    ///
    pub fn to_raw(self) -> u8 {
        match self {
            RGBSDAChannelType::Red => 0,
            RGBSDAChannelType::Green => 1,
            RGBSDAChannelType::Blue => 2,
            RGBSDAChannelType::Stencil => 13,
            RGBSDAChannelType::Depth => 14,
            RGBSDAChannelType::Alpha => 15,
        }
    }
}
