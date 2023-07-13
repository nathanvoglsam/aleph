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

use crate::utils::{find_project_file, BuildPlatform, Target};
use aleph_target::Architecture;
use anyhow::anyhow;
use std::path::PathBuf;

pub fn project_file() -> std::io::Result<PathBuf> {
    find_project_file(std::env::current_dir()?)
}

pub fn project_root() -> std::io::Result<PathBuf> {
    project_file().map(|mut v| {
        v.pop();
        v
    })
}

pub fn aleph_root() -> std::io::Result<PathBuf> {
    project_root().map(|v| v.join(".aleph"))
}

pub fn target_project_root(target: &Target) -> anyhow::Result<PathBuf> {
    assert_ne!(target.arch, Architecture::Unknown);

    match target.platform {
        BuildPlatform::UWP => {
            let mut root = aleph_root()?;
            root.push("proj");
            root.push("uwp");
            root.push(target.arch.name());
            Ok(root)
        }
        BuildPlatform::Android => {
            let mut root = aleph_root()?;
            root.push("proj");
            root.push("android");
            Ok(root)
        }
        _ => Err(anyhow!(
            "Platform \"{}\" does not have a target specific sub-project.",
            target.platform.name()
        )),
    }
}

pub fn ensure_target_project_root(target: &Target) -> anyhow::Result<PathBuf> {
    let path = target_project_root(target)?;
    std::fs::create_dir_all(&path)?;
    Ok(path)
}
