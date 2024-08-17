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

package aleph_config;

import aleph_config.BuildType;
import aleph_config.Architecture;
import aleph_config.Platform;

/**
 * Class that encapsulates a view of the game's execution for a config script to read, interpret and
 * make decisions on.
 * 
 * The runtime host is expected to construct this and provide it to the config scripts directly.
 * A user should never have to construct this directly.
 */
@:expose
class Environment {
    /**
     * The platform the game is running on
     */
    public var platform: Platform;

    /**
     * The CPU architecture the game is running on
     */
    public var arch: Architecture;

    /**
     * The build type the game was compiled and is for
     */
    public var buildType: BuildType;

    /**
     * [Description] Internal use only. Constructs a new Environment from the given values.
     * @param target 
     */
    private function new(
        platform: Platform,
        arch: Architecture,
        buildType: BuildType
    ) {
        this.platform = platform;
        this.arch = arch;
        this.buildType = buildType;
    }
    
    /**
     * [Description]
     * Exported static function with a well known name that constructs a new Environment from the
     * raw FFI level types. Internal use only.
     * 
     * This is expected to be exported in the JS module at a well-known location for the Rust glue
     * in aleph-config to find. It is the caller's responsibility to ensure the string IDs for each
     * parameter are valid for the enum types declared here.
     * 
     * @param platform The string ID of the game's build platform
     * @param arch The string ID of the game's build CPU architecture
     * @param buildType The string ID of the build type the game was compiled for
     * @return Environment
     */
    private static function create(
        platform: PlatformId,
        arch: ArchitectureId,
        buildType: BuildTypeId
    ): Environment {
        var platform = new Platform(platform);
        var arch = new Architecture(arch);
        var buildType = new BuildType(buildType);
        return new Environment(platform, arch, buildType);
    }
}