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

use std::sync::Arc;

use crate::renderer::surface_notify::ISurfaceNotify;

/// Internal state used to manage a surface shared from the host into the renderer.
pub struct SharedSurface {
    /// Handle to an [`rhi::ISurface`] that has been shared with the renderer by the host. The
    /// renderer will manage any swap chains constructed on this surface.
    pub _surface: Arc<dyn rhi::ISurface>,

    /// Receiver half of a channel that the renderer may receive notifications about changes to the
    /// surface from the host application.
    pub notify: Box<dyn ISurfaceNotify>,

    /// Swap chain constructed from 'surface' that the renderer must manage.
    pub swap_chain: Arc<dyn rhi::ISwapChain>,
}
