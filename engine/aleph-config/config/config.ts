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

/**
 * Enumeration of all supported CPU architectures.
 */
declare const enum Architecture {
    x86_64 = "x86_64",
    Aarch64 = "aarch64",
}

/**
 * Enumeration of build types the engine can be built under.
 */
declare const enum BuildType {
    /**
     * A development build. This does _not_ imply optimization level. It is perfectly valid for a
     * 'dev' build to be compiled with or without optimizations.
     * 
     * A development build means that the build may have access to development only features,
     * features that may be compiled out in a 'retail' build.
     */
    Dev = "dev",

    /**
     * Following on from a 'dev' build, a 'retail' build is a public release prepared build. A
     * retail build may have certain features compiled out.
     * 
     * This can be matched on in config scripts to disable or configure options differently for a
     * final release 'retail' build.
     */
    Retail = "retail",
}

/**
 * Enumeration of all support host platforms.
 */
declare const enum Platform {
    Windows = "windows",
    Linux = "linux",
    Macos = "macos",
    iOS = "ios",
}

// ============================================================================================== //

/**
 * Contains information about the host machine the config script is running on.
 */
declare const Environment: {
    /** The current host platform the game is running on */
    readonly platform: Platform,

    /** The type of build that is being run */
    readonly buildType: BuildType,

    /** The CPU architecture of the build that is being run */
    readonly arch: Architecture,
};

// ============================================================================================== //

declare interface Configs { }

/**
 * Global table where named config sub-tables are appended to. Each crate's suite of config scripts
 * are expected to interact with the host via this table.
 * 
 * It is convention for each config file to declare an extension to the 'Configs' interface with
 * a single extra field of some shape (typically another object). Each config file should only
 * touch Configs to add their declared table, and should not write to any other table in Configs.
 * 
 * Override scripts are an exception. An override script is a config script starting with an @ in
 * the file name. These are specifically run _after_ all normal config scripts. The expectation is
 * that override scripts must wait for the config setup scripts to build their default
 * configuration before they can modify the Configs object. Override scripts are conventionally
 * allowed to write anything.
 */
declare const Configs: Configs;
