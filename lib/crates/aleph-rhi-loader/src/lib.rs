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

pub use aleph_rhi_loader_api::ContextCreateError;
pub use aleph_rhi_loader_api::ContextOptions;
pub use aleph_rhi_loader_api::IRhiBackend;

pub struct RhiLoader {
    /// List of backends that are available
    backends: Vec<BackendAPI>,

    /// The backend object for d3d12
    d3d12: Option<&'static dyn IRhiBackend>,

    /// The backend object for vulkan
    vulkan: Option<&'static dyn IRhiBackend>,
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

        // Filter out any denied backends from the available backend set
        let allowed_backends: Vec<_> = if let Some(denied) = options.denied_backends {
            self.backends
                .iter()
                .copied()
                .filter(|v| {
                    if denied.contains(v) {
                        log::debug!("Backend '{}' denied by user", *v);
                        false
                    } else {
                        true
                    }
                })
                .collect()
        } else {
            self.backends.clone()
        };

        // First try and create a context with the explicitly required backend (if requested)
        if let Some(required) = options.required_backend {
            if !allowed_backends.contains(&required) {
                let denied = self.backends.contains(&required);
                return if denied {
                    Err(ContextCreateError::RequiredBackendDenied(required))
                } else {
                    Err(ContextCreateError::RequiredBackendUnavailable(required))
                };
            } else {
                log::debug!("Backend '{required}' chosen by user requirement");
                let backend = self.select_backend(required);
                let context = backend.make_context(options)?;
                return Ok(Self::wrap_with_validation(options, context));
            }
        }

        // Next try and create a context with the preferred API. Failing this is a soft fail.
        if let Some(preferred) = options.preferred_api {
            if allowed_backends.contains(&preferred) {
                log::debug!("Backend '{preferred}' chosen by user preference");
                let backend = self.select_backend(preferred);
                let context = backend.make_context(options)?;
                return Ok(Self::wrap_with_validation(options, context));
            } else {
                log::debug!("Preferred backend '{preferred}' not available");
            }
        }

        // Finally we use the statically preferred API on the current platform.
        if allowed_backends.contains(&Self::preferred_backend()) {
            log::debug!(
                "Backend '{}' chosen as platform default",
                Self::preferred_backend()
            );
            let backend = self.select_backend(Self::preferred_backend());
            let context = backend.make_context(options)?;
            Ok(Self::wrap_with_validation(options, context))
        } else {
            Err(ContextCreateError::NoAllowedBackendsAvailable)
        }
    }
}

impl RhiLoader {
    #[cfg(windows)]
    fn make_loader() -> Self {
        return Self {
            backends: vec![BackendAPI::D3D12, BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        };
    }

    #[cfg(target_os = "macos")]
    fn make_loader() -> Self {
        return Self {
            backends: vec![BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        };
    }

    #[cfg(target_os = "linux")]
    fn make_loader() -> Self {
        return Self {
            backends: vec![BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        };
    }

    #[cfg(target_os = "android")]
    fn make_loader() -> Self {
        return Self {
            backends: vec![BackendAPI::Vulkan],
            d3d12: None,
            vulkan: None,
        };
    }

    /// Returns the statically preferred API for the current platform
    #[cfg(windows)]
    fn preferred_backend() -> BackendAPI {
        BackendAPI::D3D12
    }

    /// Returns the statically preferred API for the current platform
    #[cfg(target_os = "macos")]
    fn preferred_backend() -> BackendAPI {
        BackendAPI::Vulkan
    }

    /// Returns the statically preferred API for the current platform
    #[cfg(target_os = "linux")]
    fn preferred_backend() -> BackendAPI {
        BackendAPI::Vulkan
    }

    /// Returns the statically preferred API for the current platform
    #[cfg(target_os = "android")]
    fn preferred_backend() -> BackendAPI {
        BackendAPI::Vulkan
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
            self.d3d12 = Some(aleph_rhi_dx12::RHI_BACKEND);
        }

        #[cfg(any(
            windows,
            target_os = "macos",
            target_os = "linux",
            target_os = "android"
        ))]
        {
            self.vulkan = Some(aleph_rhi_vulkan::RHI_BACKEND);
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
