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

use std::ffi::c_void;
use std::ptr::NonNull;

use aleph_any::{AnyArc, IAny};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use thiserror::Error;

use crate::*;

/// Represents the underlying API context. Handles creating surfaces from window handles, and
/// retrieving.
pub trait IContext: IAny + IGetPlatformInterface + Send + Sync {
    any_arc_trait_utils_decl!(IContext);

    /// Create an adapter that suitably meets the requested requirements and preferences specified
    /// by `options`. Will return `None` if no adapter meeting the requirements could be found.
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>>;

    /// Create a surface from the provided window handle.
    fn create_surface(
        &self,
        display: &dyn HasDisplayHandle,
        window: &dyn HasWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError>;

    /// Create a surface from the provided `CAMetalLayer` pointer.
    fn create_surface_for_metal_layer(
        &self,
        layer: NonNull<c_void>,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError>;

    /// Returns the API used by the underlying backend implementation.
    fn get_backend_api(&self) -> BackendAPI;
}

/// Enumeration of all available backends.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BackendAPI {
    Vulkan,
    D3D12,
    Metal,
    Null,
}

impl std::fmt::Display for BackendAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendAPI::Vulkan => f.write_str("Vulkan"),
            BackendAPI::D3D12 => f.write_str("D3D12"),
            BackendAPI::Metal => f.write_str("Metal"),
            BackendAPI::Null => f.write_str("Null"),
        }
    }
}

/// Set of errors that can occur when creating an [IContext]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ContextCreateError {
    #[error("A context has already been created by the loader")]
    ContextAlreadyCreated,

    #[error("No backends are available from the loader")]
    NoBackendsAvailable,

    #[error("The requested backend '{0}' is not available")]
    RequiredBackendUnavailable(BackendAPI),

    #[error("The context could not be created due to not meeting the minimum feature level")]
    MissingRequiredFeatures,

    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,
}

/// Set of errors that can occur when creating an [ISurface]
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SurfaceCreateError {
    #[error("An internal backend error has occurred. Details were logged.")]
    Platform,

    #[error("Requested an RHI surface for an unsupported windowing integration type.")]
    UnsupportedWSI,
}
