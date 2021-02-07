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

mod utils;

use crate::raw::windows::win32::direct3d11::D3D_FEATURE_LEVEL;
use crate::raw::windows::win32::direct3d12::{
    D3D12CreateDevice, D3D12GetDebugInterface, ID3D12Debug, ID3D12Debug1, ID3D12Device4,
};
use crate::raw::windows::win32::dxgi::{
    CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory2, DXGI_ADAPTER_DESC1,
};
use crate::raw::windows::{Abi, Error, Interface};
use crate::D3D12Object;

/// Represents the set of errors that can be encountered from device creation
#[derive(Clone, Debug, PartialEq)]
pub enum DeviceCreateError {
    /// This error occurred from DXGI
    DXGI(Error),

    /// This occurs when a compatible IDXGIAdapter was not found
    FailedToFindCompatibleAdapter,

    /// This occurs when the builder fails to create an ID3D12Device
    FailedToCreateDevice(Error),
}

/// A `Result` wrapper type used for device initialization
pub type DeviceCreateResult<T> = Result<T, DeviceCreateError>;

pub struct DeviceBuilder {
    debug: bool,
    gpu_validation: bool,
    minimum_feature_level: D3D_FEATURE_LEVEL,
}

impl DeviceBuilder {
    /// Creates a new `DeviceBuilder` instance
    ///
    /// Defaults to all debug features being disabled
    pub fn new() -> Self {
        Self {
            debug: false,
            gpu_validation: false,
            minimum_feature_level: D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_0,
        }
    }

    /// This setting controls whether we enable the DirectX12 debug layer
    ///
    /// # Warning
    ///
    /// This option will soft fail. If it was not possible to initialize the feature, the
    /// application will not report an error. This is so the interface is easier to use, as this
    /// option doesn't change any execution semantics (the side effects of the layers shouldn't be
    /// relied upon)
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// This setting controls whether to enable gpu assisted validation
    ///
    /// # Info
    ///
    /// This option will do nothing if `debug` is not also enabled
    ///
    /// # Warning
    ///
    /// This option will soft fail. If it was not possible to initialize the feature, the
    /// application will not report an error. This is so the interface is easier to use, as this
    /// option doesn't change any execution semantics (the side effects of the layers shouldn't be
    /// relied upon)
    pub fn gpu_validation(mut self, gpu_validation: bool) -> Self {
        self.gpu_validation = gpu_validation;
        self
    }

    /// This setting sets the desired minimum feature level when creating the ID3D12Device
    pub fn minimum_feature_level(mut self, minimum_feature_level: D3D_FEATURE_LEVEL) -> Self {
        self.minimum_feature_level = minimum_feature_level;
        self
    }

    pub unsafe fn build(self) -> DeviceCreateResult<Device> {
        aleph_log::trace!("Initializing D3D12 device");
        // If debug layers have been requested, we should enable them
        let debug = if self.debug {
            aleph_log::trace!("Initializing D3D12 debug layer");
            // Try to get the debug interface
            let mut debug: Option<ID3D12Debug> = None;
            let debug = D3D12GetDebugInterface(&ID3D12Debug::IID, debug.set_abi())
                .and_some(debug)
                .ok();

            // Failing to get the debug is a soft fail, so we
            if let Some(debug) = debug.as_ref() {
                debug.EnableDebugLayer();
            } else {
                aleph_log::warn!("Failed to enable D3D12 debug layer");
            }

            debug
        } else {
            None
        };

        // If gpu validation has been asked for, and is available, we will enable it
        if self.gpu_validation {
            aleph_log::trace!("Initializing D3D12 gpu validation");
            if let Some(debug) = debug.as_ref() {
                if let Ok(debug) = debug.cast::<ID3D12Debug1>() {
                    debug.SetEnableGPUBasedValidation(true.into());
                } else {
                    aleph_log::warn!("Failed to enable D3D12 gpu validation: not supported");
                }
            } else {
                aleph_log::warn!("Failed to enable D3D12 gpu validation: debug not enabled");
            }
        }

        aleph_log::trace!("Initializing IDXGIFactory");
        let mut dxgi_factory: Option<IDXGIFactory2> = None;
        let dxgi_factory = CreateDXGIFactory1(&IDXGIFactory2::IID, dxgi_factory.set_abi())
            .and_some(dxgi_factory)
            .map_err(|v| DeviceCreateError::DXGI(v))?;

        aleph_log::trace!("Selecting IDXGIAdapter");
        let adapter = utils::select_adapter(&dxgi_factory, self.minimum_feature_level)
            .ok_or(DeviceCreateError::FailedToFindCompatibleAdapter)?;

        aleph_log::trace!("Initializing ID3D12Device");
        let mut device: Option<ID3D12Device4> = None;
        let device = D3D12CreateDevice(
            Some(adapter.cast().unwrap()),
            self.minimum_feature_level,
            &ID3D12Device4::IID,
            device.set_abi(),
        )
        .and_some(device)
        .map_err(|v| DeviceCreateError::FailedToCreateDevice(v))?;

        Ok(Device {
            debug,
            dxgi_factory,
            adapter,
            device,
        })
    }
}

#[derive(Clone)]
pub struct Device {
    pub(crate) debug: Option<ID3D12Debug>,
    pub(crate) dxgi_factory: IDXGIFactory2,
    pub(crate) adapter: IDXGIAdapter1,
    pub(crate) device: ID3D12Device4,
}

impl Device {
    /// Returns a builder instance for creating a new `Device`
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::new()
    }

    pub fn raw(&self) -> &ID3D12Device4 {
        &self.device
    }

    pub fn get_adapter_desc(&self) -> raw::windows::Result<DXGI_ADAPTER_DESC1> {
        let mut desc = DXGI_ADAPTER_DESC1::default();
        unsafe { self.adapter.GetDesc1(&mut desc).ok().map(|_| desc) }
    }
}

impl D3D12Object for Device {
    unsafe fn set_name_raw(&self, name: &[u16]) -> raw::windows::Result<()> {
        self.device.SetName(name.as_ptr()).ok()
    }
}
