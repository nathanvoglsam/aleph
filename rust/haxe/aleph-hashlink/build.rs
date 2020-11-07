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
extern crate aleph_target_build as target;
extern crate regex;
extern crate semver;

use once_cell::sync::Lazy;
use regex::Regex;
use semver::Version;
use std::env;
use std::ffi::OsStr;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::process::Command;
use target::build::{host_platform, target_platform};

const ENV_LLVM_PREFIX: &str = "LLVM_SYS_PREFIX";

const LLVM_CONFIG_BINARY_NAMES: [&str; 4] = [
    "llvm-config",
    "llvm-config-11",
    "llvm-config-11.0",
    "llvm-config110",
];

const LLVM_ROOT_MARKING_FILES: [&str; 5] = [
    "LLVM-C.dll",
    "llvm-config",
    "llvm-config-11",
    "llvm-config-11.0",
    "llvm-config110",
];

const LLVM_CONFIG_PATH: Lazy<Option<PathBuf>> = Lazy::new(locate_llvm_config);
const LLVM_ROOT: Lazy<Option<PathBuf>> = Lazy::new(locate_llvm_root);

fn locate_llvm_config() -> Option<PathBuf> {
    // Either look in the directory specified by the `LLVM_SYS_PREFIX` env variable or, if it is not
    // set, look for llvm-config in the path
    let prefix = env::var_os(ENV_LLVM_PREFIX)
        .map(|p| PathBuf::from(p).join("bin"))
        .unwrap_or_else(PathBuf::new);

    // We have a list of potential binary names, look for one with a compatible version
    for binary_name in LLVM_CONFIG_BINARY_NAMES.iter() {
        let binary_name = prefix.join(binary_name);
        match llvm_version(&binary_name) {
            Ok(ref version) if is_compatible_llvm(version) => {
                // Compatible version found. Nice.
                return Some(binary_name);
            }
            Ok(version) => {
                // Version mismatch. Will try further searches, but warn that
                // we're not using the system one.
                println!(
                    "Found LLVM version {} on PATH, but need {}.",
                    version, "11.0"
                );
            }
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                // Looks like we failed to execute any llvm-config. Keep
                // searching.
            }
            // Some other error, probably a weird failure. Give up.
            Err(e) => panic!("Failed to search PATH for llvm-config: {}", e),
        }
    }

    None
}

fn locate_llvm_root() -> Option<PathBuf> {
    // First we need to decide on the path separator for the platform as this will vary
    let path_separator = if host_platform().is_windows() {
        ';'
    } else {
        ':'
    };

    // First we check if the prefix env variable points to a valid LLVM library root, as this should
    // always take priority
    let prefix = env::var_os(ENV_LLVM_PREFIX).map(|p| PathBuf::from(p));
    if let Some(prefix) = prefix {
        let bin_dir = prefix.join("bin");
        if bin_dir.is_dir() {
            for bin_name in LLVM_ROOT_MARKING_FILES.iter() {
                let bin_name = bin_dir.join(bin_name);
                if bin_name.is_file() {
                    return Some(prefix);
                }
            }
        }
    }

    // If the prefix env variable doesn't point us at an LLVM root, we search for one based on the
    // path env variable. We look for llvm-config or LLVM-C.dll, and assume if we find one of those
    // then the folder it is in is likely an LLVM install's bin dir so we can use this as our LLVM
    // root
    let path = env::var("PATH").unwrap();
    let path_iter = path
        .split(path_separator)
        .filter(|v| !v.is_empty())
        .map(|v| PathBuf::from(v))
        .filter(|p| {
            let p = p.canonicalize().unwrap();
            let cargo_target = compile::cargo_target_dir().canonicalize().unwrap();
            let cargo_target_parent = cargo_target.parent().unwrap();
            &p != &cargo_target && &p != cargo_target_parent
        });

    // Check every entry in the path, return upon the first successful match
    for check_dir in path_iter {
        for bin_name in LLVM_ROOT_MARKING_FILES.iter() {
            let bin_name = check_dir.join(bin_name);
            if bin_name.is_file() {
                return Some(check_dir.parent().unwrap().to_path_buf());
            }
        }
    }

    None
}

/// Check whether the given LLVM version is compatible with this version of
/// the crate.
fn is_compatible_llvm(llvm_version: &Version) -> bool {
    llvm_version.major == 11 && llvm_version.minor == 0
}

/// Get the output from running `llvm-config` with the given argument.
///
/// Lazily searches for or compiles LLVM as configured by the environment
/// variables.
fn llvm_config(arg: &str) -> String {
    llvm_config_ex(&LLVM_CONFIG_PATH.clone().unwrap(), arg)
        .expect("Surprising failure from llvm-config")
}

/// Invoke the specified binary as llvm-config.
///
/// Explicit version of the `llvm_config` function that bubbles errors
/// up.
fn llvm_config_ex<S: AsRef<OsStr>>(binary: S, arg: &str) -> io::Result<String> {
    Command::new(binary)
        .arg(arg)
        .arg("--link-static") // Don't use dylib for >= 3.9
        .output()
        .map(|output| {
            String::from_utf8(output.stdout).expect("Output from llvm-config was not valid UTF-8")
        })
}

/// Get the LLVM version using llvm-config.
fn llvm_version<S: AsRef<OsStr>>(binary: &S) -> io::Result<Version> {
    let version_str = llvm_config_ex(binary.as_ref(), "--version")?;

    // LLVM isn't really semver and uses version suffixes to build
    // version strings like '3.8.0svn', so limit what we try to parse
    // to only the numeric bits.
    let re = Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)(?:\.(?P<patch>\d+))??").unwrap();
    let c = re
        .captures(&version_str)
        .expect("Could not determine LLVM version from llvm-config.");

    // some systems don't have a patch number but Version wants it so we just append .0 if it isn't
    // there
    let s = match c.name("patch") {
        None => format!("{}.0", &c[0]),
        Some(_) => c[0].to_string(),
    };
    Ok(Version::parse(&s).unwrap())
}

fn get_system_libraries() -> Vec<String> {
    llvm_config("--system-libs")
        .split(&[' ', '\n'] as &[char])
        .filter(|s| !s.is_empty())
        .filter(|s| {
            // pthread is already linked else where in pc-windows-gnu, don't link it again
            let contains_pthread = s.contains("pthread");
            let windows = target_platform().is_windows();
            !(contains_pthread && windows)
        })
        .map(|flag| {
            if target_platform().is_msvc() {
                assert!(flag.ends_with(".lib"));
                &flag[..flag.len() - 4]
            } else {
                assert!(flag.starts_with("-l"));
                &flag[2..]
            }
        })
        .map(str::to_owned)
        .collect::<Vec<String>>()
}

/// Get the names of libraries to link against.
fn get_link_libraries() -> Vec<String> {
    llvm_config("--libnames")
        .split(&[' ', '\n'] as &[char])
        .filter(|s| !s.is_empty())
        .map(|name| {
            if target_platform().is_msvc() {
                assert!(name.ends_with(".lib"));
                &name[..name.len() - 4]
            } else {
                assert!(name.starts_with("lib") && name.ends_with(".a"));
                &name[3..name.len() - 2]
            }
        })
        .map(str::to_owned)
        .collect::<Vec<String>>()
}

fn is_llvm_debug() -> bool {
    // Has to be either Debug or Release
    llvm_config("--build-mode").contains("Debug")
}

fn main() {
    // Behavior can be significantly affected by these vars.
    println!("cargo:rerun-if-env-changed={}", ENV_LLVM_PREFIX);

    if LLVM_ROOT.is_none() {
        panic!("Couldn't find LLVM root");
    }

    // We link to LLVM-C.dll on MSVC
    if target_platform().is_msvc() {
        // Get llvm root and compute bin and lib path
        let root = LLVM_ROOT.clone().unwrap();
        let lib_dir = root.join("lib");
        let bin_dir = root.join("bin");

        // Link to LLVM-C
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=dylib=LLVM-C");

        let dll = bin_dir.join("LLVM-C.dll");
        compile::copy_file_to_artifacts_dir(&dll).expect(&format!(
            "Failed to copy {} into artifacts dir",
            dll.display()
        ));
        compile::copy_file_to_target_dir(&dll)
            .expect(&format!("Failed to copy {} into target dir", dll.display()));
    } else {
        if LLVM_CONFIG_PATH.is_none() {
            println!("cargo:rustc-cfg=LLVM_SYS_NOT_FOUND");
            return;
        }

        // Get the llvm library dir
        let lib_dir = llvm_config("--libdir");

        // Add library dir to libdir and link search path
        println!("cargo:libdir={}", lib_dir);
        println!("cargo:rustc-link-search=native={}", lib_dir);

        // Statically link LLVM
        for name in get_link_libraries() {
            println!("cargo:rustc-link-lib=static={}", name);
        }

        // Dynamically link to LLVM dependencies
        for name in get_system_libraries() {
            println!("cargo:rustc-link-lib=dylib={}", name);
        }

        // We also need to link against stdc++ and libffi
        println!("cargo:rustc-link-lib=dylib={}", "stdc++");
        println!("cargo:rustc-link-lib=dylib={}", "ffi");
    }
}
