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

use crate::error::ParserError;
use crate::module_file_candidates::ModuleFileCandidates;
use crate::result::Result;
use std::io::{ErrorKind, Read};
use std::path::Path;
use syn::Item;

///
/// Takes the source directory of a crate and attempts to parse the entire crate's file tree into a
/// single AST.
///
/// Depending on how big the crate is this could use quite a bit of memory.
///
/// # Example
///
/// ```
/// // Assuming you're working dir contains the Cargo.toml for a library crate
/// let path = std::env::current_dir().unwrap();
/// crate::parse_crate(path.join("src")).unwrap();
/// ```
///
pub fn parse_crate<P: AsRef<Path>>(root_dir: P) -> Result<syn::File> {
    let root_dir = root_dir.as_ref();

    // Try to open the root file of the crate, mapping not found to a more descriptive error
    let path = root_dir.join("lib.rs");
    let mut crate_root = std::fs::OpenOptions::new()
        .create(false)
        .read(true)
        .write(false)
        .open(&path)
        .map_err(|v| {
            if v.kind() == ErrorKind::NotFound {
                ParserError::CrateRootNotFound
            } else {
                ParserError::IOError(v)
            }
        })?;

    // Read the file into a string
    let mut content = String::new();
    crate_root.read_to_string(&mut content)?;

    // Parse the file
    let mut file = syn::parse_file(&content)?;

    // Recursively resolve all modules in the crate
    resolve_modules(&path, &mut file)?;

    if check_for_unresolved_modules(&file) {
        Err(ParserError::ModuleGraphInvalid)
    } else {
        Ok(file)
    }
}

///
/// Returns if there are any unresolved `Mod` items
///
fn check_for_unresolved_modules(file: &syn::File) -> bool {
    fn recurse(module: &syn::ItemMod) -> bool {
        if let Some((_, items)) = &module.content {
            for item in items.iter() {
                match item {
                    Item::Mod(module) => {
                        if recurse(module) {
                            return true;
                        }
                    }
                    _ => {}
                };
            }
            false
        } else {
            true
        }
    }

    for item in file.items.iter() {
        match item {
            Item::Mod(module) => {
                if recurse(module) {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

fn resolve_modules(path: &Path, file: &mut syn::File) -> Result<()> {
    // Iterate over all items, checking for `Mod` items.
    for item in file.items.iter_mut() {
        let item: &mut Item = item;
        match item {
            Item::Mod(module) => {
                resolve_module(path, module)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn resolve_module(parent_module_path: &Path, module: &mut syn::ItemMod) -> Result<()> {
    // If content is empty then we need to parse another file to resolve the content
    if module.content.is_none() {
        // Get the name of the module to produce a ModuleFileCandidates iterator to
        // search for the mod file
        let module_name = module.ident.to_string();
        let candidates = ModuleFileCandidates::new(&module_name);

        // We need this later in the loop and it must exist, so check for it here
        let module_parent_dir = parent_module_path.parent().unwrap();

        // Check through the list of candidates
        for candidate in candidates {
            // Build a path and try to open the candidate file
            let child_module_path = module_parent_dir.join(candidate);
            let child_module_file = std::fs::OpenOptions::new()
                .create(false)
                .read(true)
                .write(false)
                .open(&child_module_path);

            // A file not found error is not an error yet. The module file may be
            // elsewhere. This only becomes an error if we don't find *any* file from
            // the search.
            // Any other IO error is an error though so we exit out of the function with
            // the error
            let mut child_module_file = if let Err(e) = child_module_file {
                if e.kind() == ErrorKind::NotFound {
                    continue;
                } else {
                    return Err(ParserError::IOError(e));
                }
            } else {
                // We've already proven it's not Err
                child_module_file.unwrap()
            };

            // Read the file into a string
            let mut content = String::new();
            child_module_file.read_to_string(&mut content)?;

            // Parse the file
            let mut child_module_file = syn::parse_file(&content)?;

            // Must have a parent
            let child_module_parent_dir = child_module_path.parent().unwrap();

            // If the module file is next to the parent module then we wont recurse further as there
            // is an edge case that could cause infinite recursion. Instead, we bubble an error up
            // when we detect this edge case later. I wouldn't dream of trying to parse every valid
            // rust crate at this early a stage of development
            if child_module_parent_dir != module_parent_dir {
                resolve_modules(child_module_parent_dir, &mut child_module_file)?;
            }

            // Move all the attributes to the new module
            let mut attrs = child_module_file.attrs;
            module.attrs.append(&mut attrs);

            // Fill the content with the content of the file
            let brace = syn::token::Brace::default();
            let items = child_module_file.items;
            module.content = Some((brace, items));

            // Exit as we've now parsed the module
            return Ok(());
        }
        Err(ParserError::ModFileNotFound)
    } else {
        Ok(())
    }
}
