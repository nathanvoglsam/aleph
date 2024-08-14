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
class Platform {
    private var v: PlatformId;

    public function new(v: PlatformId) {
        this.v = v;
    }

    /**
     * [Description]
     * Checks if 'this' and 'other' represent the same platform.
     * 
     * This compares the internal IDs and is not a simple reference equality check.
     * @param other 
     * @return Bool
     */
    public function isSame(other: Platform): Bool {
        return this.v == other.v;
    }

    /**
     * [Description] Shortcut for comparing against WINDOWS_GNU
     * @return Bool
     */
    public function isWindowsGnu(): Bool {
        return this.isSame(Platform.WINDOWS_GNU);
    }

    /**
     * [Description] Shortcut for comparing against WINDOWS_MSVC
     * @return Bool
     */
    public function isWindowsMsvc(): Bool {
        return this.isSame(Platform.WINDOWS_MSVC);
    }

    /**
     * [Description] Checks if the target is any of the Windows PC targets (gnu or msvc)
     * @return Bool
     */
    public function isWin32(): Bool {
        return this.isWindowsGnu() || this.isWindowsMsvc();
    }

    /**
     * [Description] Shortcut for comparing against UWP_GNU
     * @return Bool
     */
    public function isUwpGnu(): Bool {
        return this.isSame(Platform.UWP_GNU);
    }

    /**
     * [Description] Shortcut for comparing against UWP_MSVC
     * @return Bool
     */
    public function isUwpMsvc(): Bool {
        return this.isSame(Platform.UWP_MSVC);
    }

    /**
     * [Description] Checks if the target is any of the Windows UWP targets (gnu or msvc)
     * @return Bool
     */
    public function isUwp(): Bool {
        return this.isUwpGnu() || this.isUwpMsvc();
    }

    /**
     * [Description] Checks if the target is any of the Windows targets (uwp or pc, gnu or msvc)
     * @return Bool
     */
    public function isWindows(): Bool {
        return this.isWin32() || this.isUwp();
    }

    /**
     * [Description] Checks if the target is any of the Windows GNU targets (uwp or pc)
     * @return Bool
     */
    public function isGnu(): Bool {
        return this.isWindowsGnu() || this.isUwpGnu();
    }

    /**
     * [Description] Checks if the target is any of the Windows MSVC targets (uwp or pc)
     * @return Bool
     */
    public function isMsvc(): Bool {
        return this.isWindowsMsvc() || this.isUwpMsvc();
    }

    /**
     * [Description] Shortcut for comparing against LINUX
     * @return Bool
     */
    public function isLinux(): Bool {
        return this.isSame(Platform.LINUX);
    }

    /**
     * [Description] Shortcut for comparing against MACOS
     * @return Bool
     */
    public function isMacos(): Bool {
        return this.isSame(Platform.MACOS);
    }

    /**
     * [Description] Shortcut for comparing against ANDROID
     * @return Bool
     */
    public function isAndroid(): Bool {
        return this.isSame(Platform.ANDROID);
    }

    /**
     * [Description] Shortcut for comparing against IOS
     * @return Bool
     */
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
