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

declare type ArchitectureId = "x86_64" | "aarch64";

declare type BuildTypeId = "dev" | "retail";

declare type PlatformId = 
      "windows-gnu"
    | "windows-msvc"
    | "uwp-gnu"
    | "uwp-msvc"
    | "linux"
    | "macos"
    | "android"
    | "ios";

// ============================================================================================== //

declare const Architecture: {
    get: () => ArchitectureId;
    isX8664: (v:ArchitectureId) => boolean;
    isAArch64: (v:ArchitectureId) => boolean;
};

// ============================================================================================== //

declare const BuildType: {
    get: () => BuildTypeId;
    isDev: (v:BuildTypeId) => boolean;
    isRetail: (v:BuildTypeId) => boolean;
};

// ============================================================================================== //

declare const Platform: {
    get: () => PlatformId;
    isWindowsGnu: (v:PlatformId) => boolean;
    isWindowsMsvc: (v:PlatformId) => boolean;
    isWin32: (v:PlatformId) => boolean;
    isUwpGnu: (v:PlatformId) => boolean;
    isUwpMsvc: (v:PlatformId) => boolean;
    isUwp: (v:PlatformId) => boolean;
    isWindows: (v:PlatformId) => boolean;
    isGnu: (v:PlatformId) => boolean;
    isMsvc: (v:PlatformId) => boolean;
    isLinux: (v:PlatformId) => boolean;
    isMacos: (v:PlatformId) => boolean;
    isAndroid: (v:PlatformId) => boolean;
    isIos: (v:PlatformId) => boolean;
};

// ============================================================================================== //

declare const Environment: {
    getConfig: () => {
        platform: PlatformId,
        buildType: BuildTypeId,
        arch: ArchitectureId,
    },
};

// ============================================================================================== //

interface Configs {}

declare const Configs: Configs;
