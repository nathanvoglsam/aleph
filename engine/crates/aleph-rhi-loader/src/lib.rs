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

use aleph_any::AnyArc;
use aleph_rhi_api::*;
use serde::Deserialize;

/// Options provided when a context is created
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct ContextOptions {
    /// Specifies a preference for a specific API to the loader. `None` denotes no preference.
    ///
    /// When a `preferred_api` is provided the loader will always choose the selected backend if it
    /// is available. If the backend is *not* available then the loader will chose another from the
    /// available backends itself.
    pub backend: BackendAPI,

    /// Whether backend API validation should be enabled.
    ///
    /// Will implicitly force the `debug` option to true if `validation` is also true as on some
    /// backends the `validation` option requires loading the same `debug` utilities to function.
    ///
    /// This flag requests that the backend should enable their backend specific API validation.
    ///
    /// This will add massive amounts of overhead and should never be enabled unless debugging the
    /// backends themselves.
    ///
    /// # Detail
    ///
    /// This is will enable w/e API validation and debug tools that are available to the backend.
    ///
    /// For Vulkan this will enable the validation layers and install a debug messenger the uses
    /// the rust `log` framework.
    ///
    /// For Direct3D 12 this will enable API validation.
    pub validation: bool,

    /// Whether backend debug utilities should be enabled. This enables debug integrations for
    /// naming objects and marking code sections to the backend's API for markup in debug tools.
    ///
    /// # Detail
    ///
    /// Basically just a request to enable `VK_EXT_debug_utils` for Vulkan without enabling
    /// validation layers. Vulkan requires `VK_EXT_debug_utils` for object naming as that is the
    /// extension that provides the naming functionality.
    pub debug: bool,

    /// A set of per-backend configs that will be used by the loader to configure the loaded backend
    /// when creating the context. Only one of these will be used, depending on what backend API
    /// was selected by the user/loader.
    pub config: BackendConfigs,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default, Deserialize)]
pub struct VulkanConfig {
    /// Force disable the VK_KHR_synchronization2 path. Intended for testing the fallback path on
    /// platforms that support sync2.
    #[serde(rename = "denySync2")]
    pub deny_sync_2: bool,
}

#[cfg(any(
    windows,
    target_os = "macos",
    target_os = "ios",
    target_os = "linux",
    target_os = "android"
))]
impl Into<aleph_rhi_vulkan::VulkanConfig> for VulkanConfig {
    fn into(self) -> aleph_rhi_vulkan::VulkanConfig {
        aleph_rhi_vulkan::VulkanConfig {
            deny_sync_2: self.deny_sync_2,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default, Deserialize)]
pub struct D3D12Config {}

#[cfg(windows)]
impl Into<aleph_rhi_dx12::D3D12Config> for D3D12Config {
    fn into(self) -> aleph_rhi_dx12::D3D12Config {
        aleph_rhi_dx12::D3D12Config {}
    }
}

#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct BackendConfigs {
    /// The config to use for the Vulkan backend
    pub vulkan: Option<VulkanConfig>,

    /// The config to use for the D3D12 backend
    pub d3d12: Option<D3D12Config>,
}

pub struct RhiLoader {
    /// List of backends that are available
    backends: Vec<BackendAPI>,

    /// The backend object for d3d12
    d3d12: Option<&'static dyn IRhiBackend>,

    /// The backend object for vulkan
    vulkan: Option<&'static dyn IRhiBackend>,
}

impl Default for RhiLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl RhiLoader {
    /// Constructs a new RhiLoader object.
    ///
    /// Will dynamically query the system to confirm whether the underlying APIs needed for each
    /// potential backend is available on the current system. Query with
    /// [RhiLoader::is_backend_available] or check [RhiLoader::backends] to see which are available.
    #[inline]
    pub fn new() -> Self {
        let mut v = Self::make_loader();
        v.try_load_backends();
        v.prune_missing_backends();
        v
    }

    /// Returns a list of all the RHI backends available from the loader
    #[inline]
    pub fn backends(&self) -> &[BackendAPI] {
        &self.backends
    }

    /// Returns whether a specific backend is available from the loader
    #[inline]
    pub fn is_backend_available(&self, backend: BackendAPI) -> bool {
        self.backends.contains(&backend)
    }

    /// Creates the RHI [IContext] object. This can only succeed once. Calling this more than once
    /// will always return Err.
    pub fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        // Check if the backends list is empty, meaning none are available
        if self.backends.is_empty() {
            return Err(ContextCreateError::NoBackendsAvailable);
        };

        // While this shouldn't be possible and would be a bug we should fail nicely for clients
        if self.vulkan.is_none() && self.d3d12.is_none() {
            log::debug!("'backends' isn't empty but no backend objects are loaded. Likely a bug");
            return Err(ContextCreateError::NoBackendsAvailable);
        }

        let backend = options.backend;
        if !self.backends.contains(&backend) {
            return Err(ContextCreateError::RequiredBackendUnavailable(backend));
        } else {
            log::debug!("Backend '{backend}' chosen");
            let backend = self.select_backend(backend);
            let context = backend.make_context(options)?;
            return Ok(Self::wrap_with_validation(options, context));
        }
    }
}

impl RhiLoader {
    #[cfg(windows)]
    fn make_loader() -> Self {
        Self {
            backends: vec![BackendAPI::D3D12, BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        }
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn make_loader() -> Self {
        return Self {
            backends: vec![BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        };
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn make_loader() -> Self {
        return Self {
            backends: vec![BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        };
    }

    /// Internal function that will attempt to load the plugins that the loader has statically
    /// configured to be available on the platform.
    ///
    /// # Important
    ///
    /// This function *must* not initialize the actual API contexts. This strictly deals with
    /// the [IRhiBackend] interface that is exported by each backend. The intention of this function
    /// is to query the backend implementations linked into the loader for their [IRhiBackend]
    /// objects and load them into the [RhiLoader]
    fn try_load_backends(&mut self) {
        #[cfg(windows)]
        {
            self.d3d12 = Some(&aleph_rhi_dx12::RHI_BACKEND_OBJECT);
        }

        #[cfg(any(
            windows,
            target_os = "macos",
            target_os = "ios",
            target_os = "linux",
            target_os = "android"
        ))]
        {
            // Except on UWP where there's no Vulkan
            if !cfg!(target_vendor = "uwp") {
                self.vulkan = Some(&aleph_rhi_vulkan::RHI_BACKEND_OBJECT);
            }
        }
    }

    /// Internal function that will prune any backends in the original `backends` set that are
    /// dynamically unavailable on the current system.
    fn prune_missing_backends(&mut self) {
        if let Ok(index) = self.backends.binary_search(&BackendAPI::D3D12) {
            if let Some(backend) = self.d3d12 {
                if !backend.is_available() {
                    log::warn!("Backend 'D3D12' is not available on current system");
                    let _ = self.backends.swap_remove(index);
                }
            } else {
                // Remove the backend for the list if we don't even have an IRhiBackend object for
                // it
                let _ = self.backends.swap_remove(index);
            }
        }

        if let Ok(index) = self.backends.binary_search(&BackendAPI::Vulkan) {
            if let Some(backend) = self.vulkan {
                if !backend.is_available() {
                    log::warn!("Backend 'Vulkan' is not available on current system");
                    let _ = self.backends.swap_remove(index);
                }
            } else {
                // Remove the backend for the list if we don't even have an IRhiBackend object for
                // it
                let _ = self.backends.swap_remove(index);
            }
        }
    }

    fn select_backend(&self, backend: BackendAPI) -> &dyn IRhiBackend {
        match backend {
            BackendAPI::Vulkan => self.vulkan.unwrap(),
            BackendAPI::D3D12 => self.d3d12.unwrap(),
            BackendAPI::Null => unimplemented!(),
        }
    }

    fn wrap_with_validation(
        options: &ContextOptions,
        context: AnyArc<dyn IContext>,
    ) -> AnyArc<dyn IContext> {
        if options.validation {
            aleph_rhi_validation::ValidationContext::wrap_context(context)
        } else {
            context
        }
    }
}

trait IRhiBackend {
    /// Returns whether the backend is dynamically available on the current system.
    ///
    /// Some platforms or backends may not always be able to provide a context, such as Vulkan. On
    /// Vulkan may be missing if `vulkan-1.dll` can't be found.
    fn is_available(&self) -> bool;

    /// Creates the RHI [IContext] object. This can only succeed once. Calling this more than once
    /// will always return `Err`.
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError>;
}

#[cfg(any(
    windows,
    target_os = "macos",
    target_os = "ios",
    target_os = "linux",
    target_os = "android"
))]
impl IRhiBackend for aleph_rhi_vulkan::VulkanLoader {
    fn is_available(&self) -> bool {
        aleph_rhi_vulkan::VulkanLoader::is_available(self)
    }

    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        let config = options.config.vulkan.clone().unwrap_or_default();
        let config = config.into();
        aleph_rhi_vulkan::VulkanLoader::make_context(
            self,
            options.validation,
            options.debug,
            &config,
        )
    }
}

#[cfg(windows)]
impl IRhiBackend for aleph_rhi_dx12::D3D12Loader {
    fn is_available(&self) -> bool {
        aleph_rhi_dx12::D3D12Loader::is_available(self)
    }

    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        let config = options.config.d3d12.clone().unwrap_or_default();
        let config = config.into();
        aleph_rhi_dx12::D3D12Loader::make_context(self, options.validation, options.debug, &config)
    }
}
