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

use aleph_any::{AnyArc, IAny};
use thiserror::Error;

use crate::*;

/// Represents some GPU device installed in the system. An adapter is used to create an [IDevice].
pub trait IAdapter: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IAdapter);

    /// Returns the [AdapterDescription] that provides information about this specific adapter.
    fn description(&self) -> AdapterDescription<'_>;

    /// Requests an IDevice
    fn request_device(&self) -> Result<AnyArc<dyn IDevice>, RequestDeviceError>;
}

/// The set of preferences that can be requested for the type of adapter to select.
#[derive(Copy, Clone, Debug)]
pub enum AdapterTypePreference {
    /// Instructs the context to prefer a hardware adapter if one is available.
    Hardware,

    /// Instructs the context to prefer a software adapter if one is available.
    Software,
}

impl Default for AdapterTypePreference {
    #[inline(always)]
    fn default() -> Self {
        Self::Hardware
    }
}

/// The set of adapter power classes. Primarily used as part of requesting an adapter from the
/// [IContext].
#[derive(Copy, Clone, Debug)]
pub enum AdapterPowerClass {
    /// A low-power adapter refers to the most power efficient GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, low-power refers to the integrated
    /// GPU as it will almost certainly use less power than the discrete GPU.
    LowPower,

    /// A high-power adapter refers to the highest performance GPU installed in the host system.
    ///
    /// e.g. In a laptop with an integrated and discrete GPU, high-power refers to the discrete GPU
    /// as it will almost certainly be faster than the integrated GPU (otherwise why would it be
    /// installed in the system?).
    HighPower,
}

impl Default for AdapterPowerClass {
    #[inline(always)]
    fn default() -> Self {
        Self::LowPower
    }
}

#[derive(Clone)]
pub struct AdapterRequestOptions<'a> {
    /// A handle to an [ISurface] which the device adapter must be able to render and present to.
    ///
    /// Can be set to `None` to indicate we aren't going to present. Useful for compute-only
    /// workloads.
    pub surface: Option<&'a dyn ISurface>,

    /// Specifies the preferred power class of the adapter the context should return. See
    /// [AdapterPowerClass] for the meaning of each power class.
    ///
    /// This only specifies a preference. There is no guarantee that the returned adapter will be
    /// of any particular power class, only that the context will chose the best available match
    /// out of the set of compatible adapters.
    ///
    /// e.g. If a system only has a single dedicated GPU and the preferred power class is low-power
    /// then the context will still yield the dedicated GPU.
    pub power_class: AdapterPowerClass,

    /// What type of device is preferred when selecting an adapter.
    pub type_preference: AdapterTypePreference,

    /// Whether to allow the implementation to select a software adapter in any capacity. This
    /// option can be used to force the context to never select software adapters, unlike
    /// 'type_preference' which is a soft request to prefer one over the other.
    pub allow_software_adapters: bool,

    /// Whether to allow the implementation to select a hardware adapter in any capacity. This
    /// option can be used to force the context to never select hardware adapters, unlike
    /// 'type_preference' which is a soft request to prefer one over the other.
    pub deny_hardware_adapters: bool,
}

impl<'a> Default for AdapterRequestOptions<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            // We can't make a "default" surface so just default to no surface.
            surface: None,

            // 99.9999% users will ask for the HighPower adapter so we default to that.
            power_class: AdapterPowerClass::HighPower,

            // Again, 99.9999% of users will ask for a hardware adapter so we default to that.
            type_preference: AdapterTypePreference::Hardware,

            // Again, 99.9999% of users will want a hard fail with no hardware adapter
            allow_software_adapters: false,

            // Again, 99.9999% of users will want hardware adapters
            deny_hardware_adapters: false,
        }
    }
}

impl<'a> std::fmt::Debug for AdapterRequestOptions<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdapterRequestOptions")
            .field("surface", &self.surface.as_ref().map(|_| "<ptr>"))
            .field("power_class", &self.power_class)
            .field("type_preference", &self.type_preference)
            .field("allow_software_adapters", &self.allow_software_adapters)
            .field("deny_hardware_adapters", &self.deny_hardware_adapters)
            .finish()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AdapterVendor {
    Unknown,
    NVIDIA,
    AMD,
    Intel,
    Apple,
    ImaginationTechnology,
    ARM,
    Qualcomm,
}

impl std::fmt::Display for AdapterVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterVendor::Unknown => f.write_str("Unknown"),
            AdapterVendor::NVIDIA => f.write_str("NVIDIA"),
            AdapterVendor::AMD => f.write_str("AMD"),
            AdapterVendor::Intel => f.write_str("Intel"),
            AdapterVendor::Apple => f.write_str("Apple"),
            AdapterVendor::ImaginationTechnology => f.write_str("ImaginationTechnology"),
            AdapterVendor::ARM => f.write_str("ARM"),
            AdapterVendor::Qualcomm => f.write_str("Qualcomm"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AdapterDescription<'a> {
    /// The name of the adapter
    pub name: &'a str,

    /// The adapter's vendor, if one could be identified
    pub vendor: AdapterVendor,
}

/// Set of errors that can occur when creating an [IDevice]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RequestDeviceError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}
