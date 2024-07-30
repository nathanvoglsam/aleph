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

enum abstract ArchitectureId(String) {
    var X8664 = "x86_64";
    var AArch64 = "aarch64";
}

/**
 * A wrapper over 'ArchitectureId' that provides an array of utility functions.
 */
@:expose
class Architecture {
    private var v: ArchitectureId;

    public function new(v: ArchitectureId) {
        this.v = v;
    }

    public function isSame(other: Architecture): Bool {
        return this.v == other.v;
    }

    public function isX8664(): Bool {
        return this.isSame(Architecture.X8664);
    }

    public function isAArch64(): Bool {
        return this.isSame(Architecture.AARCH64);
    }

    public static var X8664 = new Architecture(ArchitectureId.X8664);
    public static var AARCH64 = new Architecture(ArchitectureId.AArch64);
}
