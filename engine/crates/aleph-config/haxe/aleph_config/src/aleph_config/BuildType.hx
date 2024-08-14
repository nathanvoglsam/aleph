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

enum abstract BuildTypeId(String) {
    var Dev = "dev";
    var Retail = "retail";
}

/**
 * A wrapper over 'BuildTypeId' that provides an array of utility functions.
 */
class BuildType {
    private var v: BuildTypeId;

    public function new(v: BuildTypeId) {
        this.v = v;
    }

    /**
     * [Description]
     * Checks if 'this' and 'other' represent the same build type.
     * 
     * This compares the internal IDs and is not a simple reference equality check.
     * @param other 
     * @return Bool
     */
    public function isSame(other: BuildType): Bool {
        return this.v == other.v;
    }

    /**
     * [Description] Shortcut for comparing against DEV
     * @return Bool
     */
    public function isDev(): Bool {
        return this.isSame(BuildType.DEV);
    }

    /**
     * [Description] Shortcut for comparing against RETAIL
     * @return Bool
     */
    public function isRetail(): Bool {
        return this.isSame(BuildType.RETAIL);
    }

    public static var DEV = new BuildType(BuildTypeId.Dev);
    public static var RETAIL = new BuildType(BuildTypeId.Retail);
}
