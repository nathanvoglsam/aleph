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

use crate::raw::windows::win32::direct3d12::{
    ID3D12CommandQueue, D3D12_COMMAND_LIST_TYPE, D3D12_COMMAND_QUEUE_DESC,
    D3D12_COMMAND_QUEUE_FLAGS,
};
use raw::windows::{Abi, Interface};

#[test]
pub fn test() {
    let device = crate::DeviceBuilder::new().build().unwrap();

    unsafe {
        let desc = D3D12_COMMAND_QUEUE_DESC {
            r#type: D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT,
            priority: 0,
            flags: D3D12_COMMAND_QUEUE_FLAGS::D3D12_COMMAND_QUEUE_FLAG_NONE,
            node_mask: 0,
        };
        let mut queue: Option<ID3D12CommandQueue> = None;
        let _queue = device
            .device
            .CreateCommandQueue(&desc, &ID3D12CommandQueue::IID, queue.set_abi())
            .and_some(queue)
            .unwrap();
    }
}
