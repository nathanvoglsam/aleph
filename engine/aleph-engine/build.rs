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

extern crate aleph_compile as compile;

use aleph_target::Platform;

///
/// Main driver for compiling SDL3, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    let deps_install_path = compile::deps_path().join("install");

    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed={}", deps_install_path.display());

    let target_platform = Platform::build_target();

    let lib_path = deps_install_path.join("lib");

    match target_platform {
        Platform::WindowsGNU | Platform::WindowsMSVC => {
            println!(
                "cargo:rustc-link-search=native={}",
                lib_path.canonicalize().unwrap().display()
            );
            println!("cargo:rustc-link-lib=static=SDL3-static");

            // Hand unrolled from the generated pkg-config. These are the dependencies of
            // SDL3 that we must link to as well.
            println!("cargo:rustc-link-lib=kernel32");
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=winmm");
            println!("cargo:rustc-link-lib=imm32");
            println!("cargo:rustc-link-lib=ole32");
            println!("cargo:rustc-link-lib=oleaut32");
            println!("cargo:rustc-link-lib=version");
            println!("cargo:rustc-link-lib=uuid");
            println!("cargo:rustc-link-lib=advapi32");
            println!("cargo:rustc-link-lib=setupapi");
            println!("cargo:rustc-link-lib=shell32");
            println!("cargo:rustc-link-lib=dinput8");
        }
        Platform::MacOS => {
            println!(
                "cargo:rustc-link-search=native={}",
                lib_path.canonicalize().unwrap().display()
            );
            println!("cargo:rustc-link-lib=static=SDL3");

            // Hand unrolled from the generated pkg-config. These are the dependencies of
            // SDL3 that we must link to as well.
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=ForceFeedback");
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=GameController");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=CoreHaptics");
        }
        Platform::Linux => {
            // Nothing has to be done on linux as the most sane choice is to use the system provided
            // SDL3 lest we wake the horrible demons of distributing your own libraries on linux.
            // If it's in the distro repository, get it from there as it will probably play much
            // nicer than compiling our own.
            println!("cargo:rustc-link-lib=dylib=SDL3");
        }
        Platform::IOS => {
            // On iOS we link to a framework rather than directly to a binary. We just need to set
            // the search path, the sdl3 crate handles the linker directives.
            println!(
                "cargo:rustc-link-search=framework={}",
                lib_path.canonicalize().unwrap().display()
            );
            todo!();
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}
