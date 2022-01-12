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

mod adapter;
mod adapter_desc;
mod alpha_mode;
mod debug;
mod factory;
mod format;
mod gpu_preference;
mod sample_desc;
mod scaling;
mod swap_chain;
mod swap_chain_desc;
mod swap_chain_flags;
mod swap_effect;
mod usage_flags;

pub use adapter::Adapter;
pub use adapter_desc::AdapterDesc;
pub use alpha_mode::AlphaMode;
pub use debug::Debug;
pub use debug::DebugID;
pub use debug::DebugRLOFlags;
pub use factory::Factory;
pub use format::Format;
pub use gpu_preference::GpuPreference;
pub use sample_desc::SampleDesc;
pub use scaling::Scaling;
pub use swap_chain::SwapChain;
pub use swap_chain_desc::SwapChainDesc1;
pub use swap_chain_flags::SwapChainFlags;
pub use swap_effect::SwapEffect;
pub use usage_flags::UsageFlags;
