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

use crate::adapter::Adapter;
use crate::internal::adapter_description_decoder::adapter_description_string;
use crate::internal::create_device::create_device;
use crate::internal::debug_interface::DebugInterface;
use crate::internal::feature_support::FeatureSupport;
use crate::internal::swap_chain_creation::dxgi_create_swap_chain;
use crate::internal::unwrap;
use crate::surface::Surface;
use aleph_gpu_impl_utils::try_clone_value_into_slot;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use interfaces::platform::HasRawWindowHandle;
use parking_lot::Mutex;
use std::any::TypeId;
use std::ops::Deref;
use windows::core::Interface;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;

pub struct Context {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) debug: Option<DebugInterface>,
    pub(crate) dxgi_debug: Option<Mutex<IDXGIDebug>>,
    pub(crate) factory: Option<Mutex<IDXGIFactory2>>,
}

declare_interfaces!(Context, [IContext]);

impl IGetPlatformInterface for Context {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        let factory = self.factory.as_ref().unwrap().lock();
        if try_clone_value_into_slot::<IDXGIFactory2>(factory.deref(), out, target).is_some() {
            return Some(());
        };

        if let Some(debug) = self.dxgi_debug.as_ref() {
            let lock = debug.lock();
            if try_clone_value_into_slot::<IDXGIDebug>(lock.deref(), out, target).is_some() {
                return Some(());
            };
        }

        None
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    /// Checks if a surface is compatible with an adapter by performing a full device initialization
    /// in order to check if it would succeed.
    ///
    /// There's no other way to check if the surface can be used on the device so we just eat some
    /// overhead at init time to do this.
    fn check_surface_compatibility(
        factory: &IDXGIFactory2,
        device: &ID3D12Device,
        surface: &Surface,
    ) -> Option<()> {
        // Create a direct queue so we can create a swapchain
        let desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: 0,
            Flags: Default::default(),
            NodeMask: 0,
        };
        let queue = unsafe {
            device
                .CreateCommandQueue::<ID3D12CommandQueue>(&desc)
                .ok()?
        };

        // Create our swap chain to check if the surface is compatible
        let desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: 128,
            Height: 128,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            Stereo: BOOL::from(false),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 3,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            AlphaMode: DXGI_ALPHA_MODE_IGNORE,
            Flags: 0,
        };
        unsafe {
            dxgi_create_swap_chain(factory, &queue, &surface, &desc).ok()?;
        }

        Some(())
    }

    /// Checks if the adapter supports all the minimum required features. This requires a full
    /// device initialization because of D3D12's API.
    fn check_mandatory_features(device: ID3D12Device) -> Option<()> {
        let feature_support = FeatureSupport::new(device).ok()?;

        if feature_support.MaxSupportedFeatureLevel().0 < D3D_FEATURE_LEVEL_12_0.0 {
            log::trace!("Adapter doesn't support Feature Level 12_0");
            return None;
        }

        if !feature_support.EnhancedBarriersSupported() {
            log::trace!("Adapter doesn't support Enhanced Barriers");
            return None;
        }

        if feature_support.HighestShaderModel().0 < D3D_SHADER_MODEL_6_0.0 {
            log::trace!("Adapter doesn't support Shader Model 6.0");
            return None;
        }

        if feature_support.HighestRootSignatureVersion().0 < D3D_ROOT_SIGNATURE_VERSION_1_1.0 {
            log::trace!("Adapter doesn't support Root Signature 1.1");
            return None;
        }

        Some(())
    }

    fn adapter_meets_requirements(
        options: &AdapterRequestOptions,
        factory: &IDXGIFactory2,
        candidate: &IDXGIAdapter1,
    ) -> bool {
        let device = create_device(candidate, D3D_FEATURE_LEVEL_11_0).ok();
        let device = if let Some(device) = device {
            device
        } else {
            log::trace!("Adapter Doesn't Provide ID3D12Device10");
            return false;
        };

        if let Some(surface) = options.surface {
            let surface = unwrap::surface(surface);
            if Self::check_surface_compatibility(factory, (&device).into(), surface).is_none() {
                log::trace!("Adapter Can't Use Requested Surface");
                return false;
            }
        }

        if Self::check_mandatory_features(device.into()).is_none() {
            return false;
        }

        true
    }

    fn select_adapter(
        &self,
        options: &AdapterRequestOptions,
        factory: &IDXGIFactory2,
        mut filter: impl FnMut(&IDXGIAdapter1) -> bool,
    ) -> Option<IDXGIAdapter1> {
        let power_preference = match options.power_class {
            AdapterPowerClass::LowPower => DXGI_GPU_PREFERENCE_MINIMUM_POWER,
            AdapterPowerClass::HighPower => DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
        };
        let deny_software_adapters = !options.allow_software_adapters;
        let deny_hardware_adapters = options.deny_hardware_adapters;

        let mut selected_hardware_adapter = None;
        let mut selected_software_adapter = None;

        unsafe {
            // If possible we can explicitly ask for a "high performance" device.
            let factory_2: &IDXGIFactory2 = factory;
            let factory_6: Option<IDXGIFactory6> = factory_2.cast::<IDXGIFactory6>().ok();

            // Loop over all the available adapters
            let mut i = 0;
            loop {
                // Use the newest available interface to enumerate the adapter
                let adapter = if let Some(factory_6) = factory_6.as_ref() {
                    factory_6.EnumAdapterByGpuPreference::<IDXGIAdapter1>(i, power_preference)
                } else {
                    factory_2.EnumAdapters1(i)
                };

                // Check if we've gotten an adapter, or break from the loop if we don't as we've either hit
                // a big error or enumerated all of them already
                if let Ok(adapter) = adapter {
                    // Get the adapter description so we can decide if we want to use it or not
                    let desc = adapter.GetDesc1().unwrap();

                    let name =
                        adapter_description_string(&desc).unwrap_or_else(|| "Unknown".to_string());
                    let vendor = match desc.VendorId {
                        0x1002 => AdapterVendor::AMD,
                        0x1010 => AdapterVendor::ImaginationTechnology,
                        0x10DE => AdapterVendor::NVIDIA,
                        0x13B5 => AdapterVendor::ARM,
                        0x5143 => AdapterVendor::Qualcomm,
                        0x8086 => AdapterVendor::Intel,
                        _ => AdapterVendor::Unknown,
                    };
                    log::trace!("=====================");
                    log::trace!("Considering Adapter: ");
                    log::trace!("Vendor : {}", vendor);
                    log::trace!("Name   : {}", name);

                    // Check the flag to determine if this adapter is a software adapter
                    let is_software_adapter = (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0) != 0;
                    let is_hardware_adapter = !is_software_adapter;

                    let software_already_selected = selected_software_adapter.is_some();
                    let hardware_already_selected = selected_hardware_adapter.is_some();

                    // Determine whether an adapter with the same type (hardware/software) has
                    // already been selected
                    let already_selected = (is_software_adapter && software_already_selected)
                        || (is_hardware_adapter && hardware_already_selected);

                    // Skip if we deny the adapter based on it's adapter type
                    let deny_adapter = (deny_software_adapters && is_software_adapter)
                        || (deny_hardware_adapters && is_hardware_adapter);

                    // Skip by advancing the counter and starting the loop again
                    if deny_adapter || already_selected {
                        log::trace!("Adapter rejected by adapter type criteria");
                        i += 1;
                        continue;
                    }

                    if filter(&adapter) {
                        if is_software_adapter {
                            selected_software_adapter = Some(adapter)
                        } else {
                            selected_hardware_adapter = Some(adapter);
                        }
                    } else {
                        log::trace!("Adapter rejected for missing required capabilities");
                    }
                } else {
                    break;
                }

                i += 1;
            }

            // Which adapter we pick depends on the user's preference.
            match options.type_preference {
                AdapterTypePreference::Hardware => {
                    selected_hardware_adapter.or(selected_software_adapter)
                }
                AdapterTypePreference::Software => {
                    selected_software_adapter.or(selected_hardware_adapter)
                }
            }
        }
    }
}

impl IContext for Context {
    fn upgrade(&self) -> AnyArc<dyn IContext> {
        AnyArc::map::<dyn IContext, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }

    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>> {
        let factory = self.factory.as_ref().unwrap().lock();
        let selected_adapter = self.select_adapter(options, &factory, |candidate| {
            Self::adapter_meets_requirements(options, &factory, candidate)
        });

        if let Some(adapter) = selected_adapter {
            let device = create_device(&adapter, D3D_FEATURE_LEVEL_11_0).ok()?;

            if let Some(surface) = options.surface {
                let surface = unwrap::surface(surface);
                Self::check_surface_compatibility(&factory, &device.into(), surface)?;
            }

            let desc = unsafe {
                adapter
                    .GetDesc1()
                    .expect("Failed to get adapter description. Something very wrong")
            };
            let name = adapter_description_string(&desc).unwrap_or_else(|| "Unknown".to_string());
            let vendor = match desc.VendorId {
                0x1002 => AdapterVendor::AMD,
                0x1010 => AdapterVendor::ImaginationTechnology,
                0x10DE => AdapterVendor::NVIDIA,
                0x13B5 => AdapterVendor::ARM,
                0x5143 => AdapterVendor::Qualcomm,
                0x8086 => AdapterVendor::Intel,
                _ => AdapterVendor::Unknown,
            };

            let adapter = AnyArc::new_cyclic(move |v| Adapter {
                this: v.clone(),
                context: self.this.upgrade().unwrap(),
                name,
                vendor,
                adapter: Mutex::new(adapter),
            });
            Some(AnyArc::map::<dyn IAdapter, _>(adapter, |v| v))
        } else {
            None
        }
    }

    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        let surface = AnyArc::new_cyclic(move |v| Surface {
            this: v.clone(),
            context: self.this.upgrade().unwrap(),
            handle: window.raw_window_handle(),
            has_swap_chain: Default::default(),
        });
        Ok(AnyArc::map::<dyn ISurface, _>(surface, |v| v))
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::D3D12
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.debug = None;
        self.factory = None;
        if let Some(dxgi_debug) = &mut self.dxgi_debug {
            let dxgi_debug = dxgi_debug.get_mut();
            unsafe {
                dxgi_debug
                    .ReportLiveObjects(DXGI_DEBUG_ALL, DXGI_DEBUG_RLO_ALL)
                    .unwrap();
            }
        }
    }
}
