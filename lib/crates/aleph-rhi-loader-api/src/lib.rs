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
use thiserror::Error;

/// Options provided when a context is created
#[derive(Clone, Default, Hash, PartialEq, Eq, Debug)]
pub struct ContextOptions<'a> {
    /// Specifies a preference for a specific API to the loader. `None` denotes no preference.
    ///
    /// When a `preferred_api` is provided the loader will always choose the selected backend if it
    /// is available. If the backend is *not* available then the loader will chose another from the
    /// available backends itself.
    pub preferred_api: Option<BackendAPI>,

    /// Specifies a list of backends that should not be selected. `None` denotes no backends are
    /// denied.
    ///
    /// When a non-empty list of `denied-backends` is provided the loader is instructed to never
    /// select those backends under any circumstances. If no backends remain after pruning from the
    /// deny list then the loader will return with an error.
    pub denied_backends: Option<&'a [BackendAPI]>,

    /// Specifies the specific backend that the loader must use. `None` denotes no hard requirement.
    ///
    /// When a `required_backend` is provided the loader *must* provide a context using the
    /// requested backend API. If the requested API is not available then the loader *must* return
    /// an error, even if another backend is available.
    pub required_backend: Option<BackendAPI>,

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
}

/// Set of errors that can occur when creating an [IContext]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ContextCreateError {
    #[error("A context has already been created by the loader")]
    ContextAlreadyCreated,

    #[error("No backends are available from the loader")]
    NoBackendsAvailable,

    #[error("No allowed backends are available. All available backends have been denied")]
    NoAllowedBackendsAvailable,

    #[error("The specifically requested backend '{0}' is not available")]
    RequiredBackendUnavailable(BackendAPI),

    #[error("The specifically requested backend '{0}' was denied by the deny list")]
    RequiredBackendDenied(BackendAPI),

    #[error("The context could not be created due to not meeting the minimum feature level")]
    MissingRequiredFeatures,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}

impl From<()> for ContextCreateError {
    #[inline(always)]
    fn from(_value: ()) -> Self {
        Self::Platform
    }
}

pub trait IRhiBackend: Send + Sync {
    /// Returns which backend this [IRhiBackend] instance represents.
    fn backend(&self) -> BackendAPI;

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
