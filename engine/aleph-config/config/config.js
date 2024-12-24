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

// ============================================================================================== //

Architecture.get = function() {
    // @ts-ignore
    return THIS_ARCHITECTURE;
}

Architecture.isX8664 = function(v) {
    return v == "x86_64";
}

Architecture.isAArch64 = function(v) {
    return v == "aarch64";
}

// ============================================================================================== //

BuildType.get = function() {
    // @ts-ignore
    return THIS_BUILD_TYPE;
}

BuildType.isDev = function(v) {
    return v == "dev";
}

BuildType.isRetail = function(v) {
    return v == "retail";
}

// ============================================================================================== //

Platform.get = function() {
    // @ts-ignore
    return THIS_PLATFORM;
}

Platform.isWindowsGnu = function(v) {
    return v == "windows-gnu";
}

Platform.isWindowsMsvc = function(v) {
    return v == "windows-msvc";
}

Platform.isWin32 = function(v) {
    return Platform.isWindowsGnu(v) || Platform.isWindowsMsvc(v);
}

Platform.isUwpGnu = function(v) {
    return v == "uwp-gnu";
}

Platform.isUwpMsvc = function(v) {
    return v == "uwp-msvc";
}

Platform.isUwp = function(v) {
    return Platform.isUwpGnu(v) || Platform.isUwpMsvc(v);
}

Platform.isWindows = function(v) {
    return Platform.isWin32(v) || Platform.isUwp(v);
}

Platform.isGnu = function(v) {
    return Platform.isWindowsGnu(v) || Platform.isUwpGnu(v);
}

Platform.isMsvc = function(v) {
    return Platform.isWindowsMsvc(v) || Platform.isUwpMsvc(v);
}

Platform.isLinux = function(v) {
    return v == "linux";
}

Platform.isMacos = function(v) {
    return v == "macos";
}

Platform.isAndroid = function(v) {
    return v == "android";
}

Platform.isIos = function(v) {
    return v == "ios";
}

// ============================================================================================== //

Environment.getConfig = function() {
    return {
        platform: Platform.get(),
        buildType: BuildType.get(),
        arch: Architecture.get(),
    };
}
