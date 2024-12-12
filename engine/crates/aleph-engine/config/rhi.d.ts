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

declare namespace rhi {
    /**
     * All the supported RHI backends.
     */
    declare type Backend = "d3d12" | "vulkan"; 

    /**
     * Special options specific to the D3D12 backend.
     */
    declare interface D3D12Options {}

    /**
     * Special options specific to the Vulkan backend.
     */
    declare interface VulkanOptions {
        /** Whether to disable sync2 and force the sync2 emulation path on. **/
        denySync2: boolean;
    }
}

interface Configs {
    "rhi": {
        /** The backend that should be used **/
        api: rhi.Backend;

        /** Any options to configure the Vulkan backend, if it is loaded. **/
        vulkan?: rhi.VulkanOptions;

        /** Any options to configure the D3D12 backend, if it is loaded. **/
        d3d12?: rhi.D3D12Options;

        /** Whether to enable RHI and platform validation layers if they are available. **/
        validation: boolean;

        /** 
         * Whether debuging utilities are allowed to be initialized. Different backends have debug
         * tools only available on dev machines.
         */
        debug: boolean;
    },
}
