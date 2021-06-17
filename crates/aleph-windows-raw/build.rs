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

extern crate proc_macro2;
extern crate quote;
extern crate syn;

#[cfg(target_os = "windows")]
fn main() {
    use syn::{ItemMod, Visibility};

    windows::build!(
        Windows::Win32::Direct3D12::*,
        Windows::Win32::Dxgi::*,
        Windows::Win32::SystemServices::{
            CreateEventA,
            CreateEventW,
            WaitForSingleObject,
            WaitForMultipleObjects,
            ResetEvent,
            LoadLibraryW,
            LoadLibraryA,
            GetProcAddress,
            GetCurrentThread,
            SetThreadDescription,
            ConvertThreadToFiberEx,
            CreateFiberEx,
            DeleteFiber,
            ConvertFiberToThread,
            SwitchToFiber,
            GetPhysicallyInstalledSystemMemory,
            VirtualAlloc,
            VirtualFree,
        },
        Windows::Win32::StructuredStorage::{
            STATSTG,
            ISequentialStream,
            IStream
        },
        Windows::Win32::WindowsProgramming::{
            CloseHandle,
            FILETIME,
        },
        Windows::Win32::Com::IMalloc,
    );

    // Get a path to where windows-rs outputs the bindings
    let src_file = bindings_file_path();

    // Get a path to where we want to output our transformation
    let output_dir = transformation_output_dir_path();

    // Read the file to a string and parse into a `syn::File`
    let file = std::fs::read_to_string(&src_file).unwrap();
    let file = syn::parse_file(&file).unwrap();

    // Delete the old bindings directory so that we don't end up with old files polluting the tree
    delete_old_bindings();

    // Extract information into a dummy module to kick off the generation process
    let attrs = file.attrs;
    let items = file.items;
    let module = ItemMod {
        attrs,
        vis: Visibility::Inherited,
        mod_token: Default::default(),
        ident: syn::Ident::new("raw", proc_macro2::Span::call_site()),
        content: Some((Default::default(), items)),
        semi: None,
    };

    // Recursively walk the module tree
    handle_module(&output_dir, Vec::new(), &module);

    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(target_os = "windows")]
fn handle_module(output_dir: &str, mut name_stack: Vec<String>, module: &syn::ItemMod) {
    // Add the module's name to the stack
    name_stack.push(module.ident.to_string());

    // Create the directory path
    let mut path = std::path::PathBuf::from(output_dir);
    for name in name_stack.iter() {
        path.push(name);
    }

    // Create all the directories so we can output the file
    std::fs::create_dir_all(&path).unwrap();

    // Recurse on the defined modules, removing the bodies so we just output `pub mod thing`
    let mut out_module = module.clone();
    if let Some(content) = out_module.content.as_mut() {
        let items = std::mem::take(&mut content.1);
        content.1 = items
            .into_iter()
            .map(|v| match v {
                syn::Item::Mod(mut module) => {
                    handle_module(&output_dir, name_stack.clone(), &module);
                    module.content = None;
                    syn::Item::Mod(module)
                }
                _ => v,
            })
            .collect();
    }

    // Now we want to output to mod.rs
    path.push("mod.rs");

    write_module_file(path, out_module);
}

#[cfg(target_os = "windows")]
fn write_module_file<P: AsRef<std::path::Path>>(file: P, module: syn::ItemMod) {
    use quote::ToTokens;

    let mut attrs = module.attrs.clone();
    for attr in attrs.iter_mut() {
        attr.style = syn::AttrStyle::Inner(Default::default())
    }
    let items = if let Some((_, items)) = module.content {
        items
    } else {
        Vec::new()
    };
    let out = syn::File {
        shebang: None,
        attrs,
        items,
    };

    let text = out.into_token_stream().to_string();
    let text = rustfmt_string(&text);

    std::fs::write(file, text).unwrap();
}

#[cfg(target_os = "windows")]
fn rustfmt_string(text: &str) -> String {
    use std::io::Write;
    use std::process::Stdio;

    let mut proc = std::process::Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let stdin = proc.stdin.as_mut().unwrap();
    stdin.write_all(text.as_bytes()).unwrap();

    let output = proc.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[cfg(target_os = "windows")]
fn bindings_file_path() -> String {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let src_file = "windows.rs";
    format!("{}\\{}", out_dir, src_file)
}

#[cfg(target_os = "windows")]
fn transformation_output_dir_path() -> String {
    let output_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    format!("{}\\src", output_dir)
}

#[cfg(target_os = "windows")]
fn delete_old_bindings() {
    let output_dir = transformation_output_dir_path();
    let mut output_dir = std::path::PathBuf::from(output_dir);
    output_dir.push("raw");

    std::fs::remove_dir_all(output_dir).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
