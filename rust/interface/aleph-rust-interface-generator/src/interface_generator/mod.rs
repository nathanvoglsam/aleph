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

use crate::error::GeneratorError;
use crate::result::Result;
use aleph_interface_description::utils::drill_through_parens;
use aleph_interface_description::{Class, Function, InterfaceDescription, Type};
use std::collections::HashMap;
use syn::export::Span;

pub struct InterfaceGenerator {
    namespace_stack: Vec<String>,
    description: InterfaceDescription<String>,
}

// TODO: Handle `use` statements
// TODO: Handle resolving paths against the current in scope names
// TODO: Handle trying to export a non #[aleph::interface] marked type (i.e as a struct member)
// TODO: Investigate wrapping tuples
// TODO: Investigate wrapping arrays as return values (move semantics)
// TODO: Investigate customization flags for interop
// TODO: Investigate opaque types

impl InterfaceGenerator {
    pub fn new() -> Self {
        Self {
            namespace_stack: Vec::new(),
            description: InterfaceDescription::default(),
        }
    }

    pub fn generate(mut self, file: syn::File) -> Result<InterfaceDescription<String>> {
        let aleph_interface = Self::aleph_interface_path();

        let mut item_iter_stack = Vec::new();

        // We need to make sure we've resolved all the structs before we try and resolve any of the
        // impl blocks on them
        item_iter_stack.push(file.items.iter());
        'struct_outer: while let Some(mut items) = item_iter_stack.pop() {
            while let Some(item) = items.next() {
                match item {
                    syn::Item::Struct(item) => {
                        // If the struct as the `#[aleph::interface]` attribute we should generate an
                        // interface for it
                        if item.attrs.iter().any(|attr| &attr.path == &aleph_interface) {
                            self.generate_struct_interface(item)?;
                        }
                    }
                    syn::Item::Mod(item) => {
                        if let Some(content) = item.content.as_ref() {
                            // Add the module name to the namespace stack
                            let mod_name = item.ident.to_string();
                            self.namespace_stack.push(mod_name);

                            // Backup the current iterator onto the iterator stack
                            item_iter_stack.push(items);

                            // Push the item iterator for the module we want to handle onto the stack
                            item_iter_stack.push(content.1.iter());

                            continue 'struct_outer;
                        }
                    }
                    _ => {}
                }
            }
            self.namespace_stack.pop();
        }

        // Assert these are empty (the algorithm should leave them empty once finished)
        debug_assert!(item_iter_stack.is_empty());
        debug_assert!(self.namespace_stack.is_empty());

        // Now we can iterate through all the bare impl blocks that refer to type's we've defined
        item_iter_stack.push(file.items.iter());
        'mod_outer: while let Some(mut items) = item_iter_stack.pop() {
            while let Some(item) = items.next() {
                match item {
                    syn::Item::Impl(item) => {
                        self.generate_impl_interface(item)?;
                    }
                    syn::Item::Mod(item) => {
                        if let Some(content) = item.content.as_ref() {
                            // Add the module name to the namespace stack
                            let mod_name = item.ident.to_string();
                            self.namespace_stack.push(mod_name);

                            // Backup the current iterator onto the iterator stack
                            item_iter_stack.push(items);

                            // Push the item iterator for the module we want to handle onto the stack
                            item_iter_stack.push(content.1.iter());

                            continue 'mod_outer;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(self.description)
    }

    fn generate_impl_interface(&mut self, item: &syn::ItemImpl) -> Result<()> {
        let self_ty = item.self_ty.as_ref();
        let self_ty = drill_through_parens(self_ty);

        // A path is the only thing we care about, every other type doesn't make sense to try and
        // export functions for
        if let syn::Type::Path(path) = self_ty {
            // Convert to our path format and reject primitive types as we can't export interfaces
            // for those
            if let Some(Type::Path(path)) = Type::from_path(&path.path) {
                let path = self.current_namespace() + &path;
                // We only want to handle impl blocks for structs we're indexing
                if let Some(class) = self.description.classes.get_mut(&path) {
                    for item in item.items.iter() {
                        if let syn::ImplItem::Method(method) = item {
                            let function = Function::from_function_signature(&method.sig);
                            if let Some(function) = function {
                                let name = method.sig.ident.to_string();
                                class.functions.insert(name, function);
                            } else {
                                return Err(GeneratorError::UnsupportedMethodSignature);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn generate_struct_interface(&mut self, item: &syn::ItemStruct) -> Result<()> {
        // Get the current namespace, fully qualified and with a trailing separator ready for the
        // name of the struct to be appended
        let namespace = self.current_namespace();
        let struct_name = item.ident.to_string();

        // Append namespace and struct name
        let struct_name = namespace + &struct_name;

        let mut members = HashMap::new();

        // Build the class description from the
        for field in item.fields.iter() {
            // Safe to unwrap here, we're only going to be parsing regular structs
            let field_name = field.ident.as_ref().unwrap().to_string();

            let ty = if let Some(ty) = Type::from_struct_field(&field.ty) {
                ty
            } else {
                return Err(GeneratorError::UnsupportedStructField);
            };

            members.insert(field_name, ty);
        }

        let class = Class {
            fields: members,
            functions: HashMap::new(),
        };

        self.description.classes.insert(struct_name, class);

        Ok(())
    }

    /// Internal function, returns a fully qualified name for the current namespace. Will leave a
    /// trailing namespace separator in the string when there is more than 0 namespace segments
    fn current_namespace(&self) -> String {
        let mut namespace = String::new();

        for i in 0..self.namespace_stack.len() {
            namespace.push_str(&self.namespace_stack[i]);
            namespace.push('.');
        }

        namespace
    }

    /// Internal function for getting the path "aleph::interface"
    fn aleph_interface_path() -> syn::Path {
        // Common identifiers
        let aleph = syn::Ident::new("aleph", Span::call_site());
        let interface = syn::Ident::new("interface", Span::call_site());

        // Make path for `aleph::interface`
        let mut aleph_interface = syn::Path::from(aleph);
        aleph_interface.segments.push(interface.into());

        aleph_interface
    }
}
