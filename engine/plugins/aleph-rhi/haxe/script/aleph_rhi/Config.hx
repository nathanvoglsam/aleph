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

package aleph_rhi;

/**
 * All the supported RHI backends.
 */
enum abstract RhiBackend(String) {
    var D3D12 = "d3d12";
    var VULKAN = "vulkan";
}

/**
 * Special options specific to the D3D12 backend.
 */
typedef D3D12Options = {}

/**
 * Special options specific to the Vulkan backend.
 */
typedef VulkanOptions = {
    /** Whether to disable sync2 and force the sync2 emulation path on. **/
    var denySync2: Bool;
}

/**
 * Collection of all options for configuring the RHI.
 */
typedef RhiConfig = {
    /** The backend that should be used **/
    var api: RhiBackend;

    /** Any options to configure the Vulkan backend, if it is loaded. **/
    var ?vulkan: VulkanOptions;

    /** Any options to configure the D3D12 backend, if it is loaded. **/
    var ?d3d12: D3D12Options;

    /** Whether to enable RHI and platform validation layers if they are available. **/
    var validation: Bool;

    /** 
     * Whether debuging utilities are allowed to be initialized. Different backends have debug
     * tools only available on dev machines.
     */
    var debug: Bool;
}

class Config {
    /**
     * [Description]
     * Utility function for fetching the 'RhiConfig' object from the given ConfigTable.
     * 
     * This is expected to be used by an ConfigOverride to fetch the current config state so it can
     * be tweaked.
     * @param config 
     * @return RhiConfig
     */
    @:access(aleph_config.ConfigTable.get)
    public static function fetch(config: aleph_config.ConfigTable): RhiConfig {
        return config.get("aleph-rhi");
    }
}
