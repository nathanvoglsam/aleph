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

package aleph_target;

enum abstract PlatformId(String) {
    var WindowsGNU = "windows-gnu";
    var WindowsMSVC = "windows-msvc";
    var UwpGNU = "uwp-gnu";
    var UwpMSVC = "uwp-msvc";
    var Linux = "linux";
    var MacOS = "macos";
    var Android = "android";
    var Ios = "ios";
}

/**
 * A wrapper over 'PlatformId' that provides an array of utility functions.
 */
@:expose
class Platform {
    private var v: PlatformId;

    public function new(v: PlatformId) {
        this.v = v;
    }

    public function isSame(other: Platform): Bool {
        return this.v == other.v;
    }

    public function isWindowsGnu(): Bool {
        return this.isSame(Platform.WINDOWS_GNU);
    }

    public function isWindowsMsvc(): Bool {
        return this.isSame(Platform.WINDOWS_MSVC);
    }

    public function isWin32(): Bool {
        return this.isWindowsGnu() || this.isWindowsMsvc();
    }

    public function isUwpGnu(): Bool {
        return this.isSame(Platform.UWP_GNU);
    }

    public function isUwpMsvc(): Bool {
        return this.isSame(Platform.UWP_MSVC);
    }

    public function isUwp(): Bool {
        return this.isUwpGnu() || this.isUwpMsvc();
    }

    public function isWindows(): Bool {
        return this.isWin32() || this.isUwp();
    }

    public function isGnu(): Bool {
        return this.isWindowsGnu() || this.isUwpGnu();
    }

    public function isMsvc(): Bool {
        return this.isWindowsMsvc() || this.isUwpMsvc();
    }

    public function isLinux(): Bool {
        return this.isSame(Platform.LINUX);
    }

    public function isMacos(): Bool {
        return this.isSame(Platform.MACOS);
    }

    public function isAndroid(): Bool {
        return this.isSame(Platform.ANDROID);
    }

    public function isIos(): Bool {
        return this.isSame(Platform.IOS);
    }

    public static var WINDOWS_GNU = new Platform(PlatformId.WindowsGNU);
    public static var WINDOWS_MSVC = new Platform(PlatformId.WindowsMSVC);
    public static var UWP_GNU = new Platform(PlatformId.UwpGNU);
    public static var UWP_MSVC = new Platform(PlatformId.UwpMSVC);
    public static var LINUX = new Platform(PlatformId.Linux);
    public static var MACOS = new Platform(PlatformId.MacOS);
    public static var ANDROID = new Platform(PlatformId.Android);
    public static var IOS = new Platform(PlatformId.Ios);
}
