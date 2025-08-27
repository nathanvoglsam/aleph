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

use aleph_rhi_api::*;
use aleph_rhi_impl_utils::conversion_function;

use crate::{
    NullBindingSignature, NullCommandList, NullDevice, NullParameterBlockLayout, NullSurface,
    NullSwapChain, NullSwapImage,
};

conversion_function!(
    ICommandList,
    NullCommandList,
    command_list,
    command_list_owned,
    command_list_d,
    command_list_iter
);
conversion_function!(
    IDevice,
    NullDevice,
    device,
    device_owned,
    device_d,
    device_iter
);
conversion_function!(
    ISurface,
    NullSurface,
    surface,
    surface_owned,
    surface_d,
    surface_iter
);
conversion_function!(
    ISwapChain,
    NullSwapChain,
    swap_chain,
    swap_chain_owned,
    swap_chain_d,
    swap_chain_iter
);
conversion_function!(
    ISwapImage,
    NullSwapImage,
    swap_image,
    swap_image_owned,
    swap_image_d,
    swap_image_iter
);
conversion_function!(
    IParameterBlockLayout,
    NullParameterBlockLayout,
    parameter_block_layout,
    parameter_block_layout_owned,
    parameter_block_layout_d,
    parameter_block_layout_iter
);
conversion_function!(
    IBindingSignature,
    NullBindingSignature,
    binding_signature,
    binding_signature_owned,
    binding_signature_d,
    binding_signature_iter
);
