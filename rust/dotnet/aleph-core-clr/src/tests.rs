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

use crate::{Delegate, Library, Runtime};
use std::path::PathBuf;

#[test]
fn test_get_delegate() {
    //let clr_path = "C:\\Program Files\\dotnet\\shared\\Microsoft.NETCore.App\\3.1.6";
    let clr_path = "D:\\Code\\dotnet-runtime-3.1.7-win-x64\\shared\\Microsoft.NETCore.App\\3.1.7";
    let assembly_path = "D:\\Code\\DelegateTest\\publish";

    let library = Library::new_in_path(clr_path).unwrap();

    let assemblies: Vec<PathBuf> = std::fs::read_dir(clr_path)
        .unwrap()
        .filter_map(|v| {
            let v = v.unwrap();
            let path = v.path();
            if let Some(extension) = path.extension() {
                if extension == "dll" {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    let app_paths = [assembly_path];

    let runtime = Runtime::builder()
        .trusted_platform_assemblies(assemblies.iter().map(|v| v.to_str().unwrap()))
        .app_paths(app_paths.iter())
        .build(&library, "DelegateTest")
        .unwrap();

    type DelegateFn = unsafe extern "C" fn() -> i32;
    let delegate: Delegate<DelegateFn> = unsafe {
        runtime
            .create_delegate(
                "DelegateTest, Version=1.0.0.0",
                "DelegateTest.DelegateTest",
                "Main",
            )
            .unwrap()
    };

    let result = unsafe { (*delegate)() };
    assert_eq!(result, 56, "Delegate returned wrong result");
}
