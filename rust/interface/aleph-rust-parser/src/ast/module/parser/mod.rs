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

use crate::ast::module::iter::{IterUnion, IterUnionMut};
use crate::ast::module::ModuleItemMut;
use crate::ast::{Class, GeneratorError, Import, Module, Path, Result};
use crate::interner::{Interner, StrId};
use aleph_interface_description::Type;
use std::collections::HashMap;
use syn::export::Span;
use syn::{UseTree, Visibility};

// Functions for constructing a module
impl Module {
    /// Parses a syn::File into our own simplified and further resolved AST
    pub fn from_file(interner: &mut Interner, file: &syn::File) -> Result<Self> {
        let mut namespace_stack = Vec::new();
        let mut item_iter_stack = Vec::new();
        let mut module_stack = Vec::new();

        // Iterate over the file and resolve all use statements
        module_stack.push(Module::default());
        item_iter_stack.push(file.items.iter());
        Self::use_resolve_stage(
            interner,
            &mut namespace_stack,
            &mut item_iter_stack,
            &mut module_stack,
        )?;

        // Validate that there are no name collisions between structs and imported objects
        Self::use_collision_validation_stage(
            module_stack.last().unwrap(), // Must exist
        )?;

        // Pass back over the module tree and resolve all use statements to a fully qualified path,
        // prune imports from external crates and enforce that all uses refer to a valid object
        Self::use_fixup_stage(
            module_stack.last_mut().unwrap(), // Must exist
            interner,
        )?;

        Ok(module_stack.pop().unwrap())
    }

    fn use_resolve_stage(
        interner: &mut Interner,
        namespace_stack: &mut Vec<StrId>,
        item_iter_stack: &mut Vec<std::slice::Iter<syn::Item>>,
        module_stack: &mut Vec<Module>,
    ) -> Result<()> {
        let aleph_interface = Self::aleph_interface_path();

        'outer: while let Some(mut items) = item_iter_stack.pop() {
            // Get the current module we'll be parsing into
            let mut internal_module = module_stack.pop().unwrap();

            while let Some(item) = items.next() {
                match item {
                    syn::Item::Use(item) => {
                        if item.leading_colon.is_some() {
                            return Err(GeneratorError::InvalidUsePath);
                        } else {
                            let public = Self::resolve_visibility(&item.vis)?;
                            Self::resolve_use_into(
                                interner,
                                &mut internal_module.imports,
                                None,
                                public,
                                &item.tree,
                            )?;
                        }
                    }
                    syn::Item::Mod(item) => {
                        if let Some(content) = item.content.as_ref() {
                            // Add the module name to the namespace stack
                            let mod_name = item.ident.to_string();
                            let mod_name = interner.intern(&mod_name);
                            namespace_stack.push(mod_name);

                            // Backup the current iterator and module and push the new ones for the
                            // next iteration
                            item_iter_stack.push(items);
                            item_iter_stack.push(content.1.iter());
                            module_stack.push(internal_module);
                            module_stack.push(Module::default());

                            continue 'outer;
                        }
                    }
                    syn::Item::Struct(item) => {
                        if item.attrs.iter().any(|attr| &attr.path == &aleph_interface) {
                            let public = Self::resolve_visibility(&item.vis)?;
                            let struct_name = item.ident.to_string();
                            let struct_name = interner.intern(struct_name);

                            let mut class = Class {
                                inner: Default::default(),
                                public,
                            };

                            for field in item.fields.iter() {
                                let field_name = field.ident.as_ref().unwrap().to_string();

                                let ty = if let Some(ty) = Type::from_struct_field(&field.ty) {
                                    ty
                                } else {
                                    return Err(GeneratorError::UnsupportedStructField);
                                };

                                class.inner.fields.insert(field_name, ty);
                            }

                            internal_module.structs.insert(struct_name, class);
                        }
                    }
                    _ => {}
                }
            }

            // If there's still a module on the module stack we should insert the module we just
            // parsed as a sub module. Otherwise we've walked the full module graph and we pop the
            // root module back on to the stack so we can get it outside the loop
            if !module_stack.is_empty() {
                let mod_name = *namespace_stack.last().unwrap();
                module_stack
                    .last_mut()
                    .unwrap()
                    .sub_modules
                    .insert(mod_name, internal_module);
            } else {
                module_stack.push(internal_module);
            }

            namespace_stack.pop();
        }
        Ok(())
    }

    fn use_collision_validation_stage(module: &Module) -> Result<()> {
        let mut stack = Vec::new();
        stack.push(module);
        while let Some(module) = stack.pop() {
            // Make sure no `uses` import colliding names
            for (name, _) in module.imports.iter() {
                if module.structs.contains_key(name) {
                    return Err(GeneratorError::MultipleObjectsWithSameName);
                }
                if module.sub_modules.contains_key(name) {
                    return Err(GeneratorError::MultipleObjectsWithSameName);
                }
            }

            // Make sure no module name collides with a struct name
            for (name, sub_module) in module.sub_modules.iter() {
                if module.structs.contains_key(name) {
                    return Err(GeneratorError::MultipleObjectsWithSameName);
                }
                stack.push(sub_module);
            }
        }
        Ok(())
    }

    fn use_fixup_stage(root: &mut Module, interner: &mut Interner) -> Result<()> {
        // The first pass resolves all paths to be fully qualified but may not actually be valid
        // paths to an object (module or struct)
        Self::use_fixup_stage_pass_1(root, interner)?;

        // The second pass will validate that all use statements that refer to objects within the
        // crate refer to an object. This should mean any path not prefixed with crate:: must refer
        // to an external crate and can be ignored
        Self::use_fixup_stage_pass_2(root, interner)?;

        Ok(())
    }

    fn use_fixup_stage_pass_1(root: &mut Module, interner: &mut Interner) -> Result<()> {
        // Intern some identifiers that we'll need
        let crate_ident = interner.intern("crate");
        let super_ident = interner.intern("super");
        let self_ident = interner.intern("self");

        // Set up our on heap stack for our depth first traversal
        let mut name_stack = vec![];
        let mut iter_stack: Vec<IterUnionMut> = vec![IterUnionMut::Root(Some((crate_ident, root)))];

        'outer: while let Some(mut iter) = iter_stack.pop() {
            while let Some((module_name, module)) = iter.next() {
                // Build of list of `use` declarations that could potentially be culled (relative
                // paths)
                let mut to_remove = Vec::new();

                // Pass one, resolve all simple cases
                for (u_name, u) in module.imports.iter_mut() {
                    // The first segment will let us decide where the path intends to begin
                    // resolving from
                    let first = *u.concrete.segments.first().unwrap();
                    if first == super_ident {
                        // "super" means to begin resolving from the parent module so substitute the
                        // old path with an absolute path that resolves from the crate root
                        let mut new_path = clone_namespace_stack(&name_stack);
                        new_path.extend_from_slice(&u.concrete.segments[1..]);
                        u.concrete.segments = new_path;
                        u.concrete.absolute = true;
                    } else if first == self_ident {
                        // "self" in a use statement means to resolve from within the module itself
                        // so replace the old "self" path with an absolute path
                        let mut new_path = clone_namespace_stack(&name_stack);
                        new_path.push(module_name);
                        new_path.extend_from_slice(&u.concrete.segments[1..]);
                        u.concrete.segments = new_path;
                        u.concrete.absolute = true;
                    } else if first == crate_ident {
                        // The only other special path segment is "crate" which means to start from
                        // the crate root. We handle absolute paths differently in our internal path
                        // type so we remove the "crate" segment and mark as absolute
                        u.concrete.segments = u.concrete.segments[1..].to_vec();
                        u.concrete.absolute = true;
                    } else {
                        // The only other special path segment left is "crate" which means to
                        // start from the crate root. We're trying to make everything be an
                        // absolute path so we just ignore anything that's already absolute. The
                        // only remaining possibilities is a path without a special search specifier
                        // which we need to handle specially.
                        to_remove.push((*u_name, first));
                    }
                }

                // Now that we know what candidates there are to remove we can iterate over them and
                // remove the ones that should be removed. We want to remove any relative path that
                // *may* resolve to an external crate. This can be deduced by whether there are any
                // objects in scope accessible by the same name as the first segment of a use path.
                // The object could be a sub module or another use. This stage isn't about ensuring
                // that there's an object at the end of the chain, just to remove anything that
                // could be referring to external crates so we can check if any types are trying to
                // generate interfaces for have external crate's stuff in any of their signatures.
                for (remove_name, requires_name) in to_remove {
                    let used_into_scope = module.imports.contains_key(&requires_name);
                    let sub_mod_in_scope = module.sub_modules.contains_key(&requires_name);
                    if !(used_into_scope || sub_mod_in_scope) {
                        module.imports.remove(&remove_name);
                    }
                }

                name_stack.push(module_name);
                iter_stack.push(iter);
                iter_stack.push(IterUnionMut::Map(module.sub_modules.iter_mut()));
                continue 'outer;
            }
            name_stack.pop();
        }

        Ok(())
    }

    fn use_fixup_stage_pass_2(root: &mut Module, interner: &mut Interner) -> Result<()> {
        // Intern some identifiers that we'll need
        let crate_ident = interner.intern("crate");

        // Set up our on heap stack for our depth first traversal
        let mut name_stack = vec![];
        let mut iter_stack: Vec<IterUnion> = vec![IterUnion::Root(Some((crate_ident, root)))];

        // We need a list of the use names to patch so we can patch them in a separate pass. To
        // satisfy the borrow checker we can't mutate any of the modules while we're iterating over
        // them so we push the actual path patching to a second pass where we can keep the borrow
        // checker happy
        let mut to_patch = Vec::new();

        'outer: while let Some(mut iter) = iter_stack.pop() {
            while let Some((module_name, module)) = iter.next() {
                // ABSOLUTE PATH TO MODULE
                let mut module_path = clone_namespace_stack(&name_stack);
                if module_name != crate_ident {
                    module_path.push(module_name);
                }
                let module_path = Path::new(module_path, true);
                //let module_path_str = module_path.to_string(interner);

                // Iterate over all the uses to find the ones we need to resolve to absolute paths
                for (u_name, u) in module.imports.iter() {
                    // We only want to operate on relative paths
                    if !u.concrete.absolute {
                        let mut new_path = module_path.segments.clone();
                        new_path.extend_from_slice(&u.concrete.segments);
                        let new_path = Path::new(new_path, true);
                        //let new_path_str = new_path.to_string(interner);

                        // OBJECT LOOKUP RESOLUTION
                        // Lookup the underlying object (resolving through all chained imports)
                        if let Some((_, _)) = root.lookup_object(&new_path) {
                            // Build a path to the actual import itself
                            let mut import_path = module_path.segments.clone();
                            import_path.push(*u_name);
                            let import_path = Path::new(import_path, true);
                            //let import_path_str = import_path.to_string(interner);

                            // Add this import to the list of imports to patch alongside the path to
                            // patch onto the import
                            to_patch.push((import_path, new_path));
                        } else {
                            // If we couldn't find anything at the end of the import chain it means
                            // that the object is not defined in the crate and we should surface an
                            // error
                            return Err(GeneratorError::ImportedObjectDoesNotExist);
                        }
                    }
                }

                name_stack.push(module_name);
                iter_stack.push(iter);
                iter_stack.push(IterUnion::Map(module.sub_modules.iter()));
                continue 'outer;
            }
            name_stack.pop();
        }

        for (import_path, new_path) in to_patch {
            match root.lookup_mut(&import_path).unwrap() {
                ModuleItemMut::Import((_, u)) => {
                    u.concrete = new_path;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
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

    fn resolve_visibility(vis: &syn::Visibility) -> Result<bool> {
        match vis {
            Visibility::Public(_) => Ok(true),
            Visibility::Crate(_) => unimplemented!(),
            Visibility::Restricted(i) => {
                if i.in_token.is_some() {
                    Err(GeneratorError::UnsupportedVisibility)
                } else {
                    if let Some(ident) = i.path.segments.first() {
                        let ident = ident.ident.to_string();
                        if &ident == "crate" {
                            Ok(true)
                        } else if &ident == "self" {
                            Ok(false)
                        } else if &ident == "super" {
                            Err(GeneratorError::UnsupportedVisibility)
                        } else {
                            Err(GeneratorError::InvalidUseSyntax)
                        }
                    } else {
                        Err(GeneratorError::InvalidUseSyntax)
                    }
                }
            }
            Visibility::Inherited => Ok(false),
        }
    }

    fn resolve_use_into(
        interner: &mut Interner,
        into: &mut HashMap<StrId, Import>,
        prefix: Option<Path>,
        public: bool,
        item: &syn::UseTree,
    ) -> Result<()> {
        // Represents the states for the internal parser state machine
        enum State {
            Path,
            Tail,
        }

        let mut path = prefix.unwrap_or_default();
        let mut current = item;

        // Resolve the initial state (and throw errors if invalid syntax detected)
        let mut state = match current {
            UseTree::Path(_) => State::Path,
            UseTree::Name(_) | UseTree::Rename(_) | UseTree::Glob(_) => {
                if path.segments.is_empty() {
                    return Err(GeneratorError::InvalidUseSyntax);
                } else {
                    State::Tail
                }
            }
            UseTree::Group(_) => State::Tail,
        };

        // This is a recursive function flattened into a loop and is a hand written parser state
        // machine so may look a little chaotic
        loop {
            match state {
                State::Path => match current {
                    // If we've got another path push it's segment onto the path we're currently
                    // building and update the `current` to point to the next item in the chain
                    UseTree::Path(i) => {
                        // Get the segment's ident as a string and intern it and push to the current
                        // path
                        let seg = i.ident.to_string();
                        let seg = interner.intern(seg);
                        path.segments.push(seg);

                        // Step to the next node in the use tree
                        current = &i.tree;
                    }
                    UseTree::Name(_) => state = State::Tail,
                    UseTree::Rename(_) => state = State::Tail,
                    UseTree::Glob(_) => state = State::Tail,
                    UseTree::Group(_) => state = State::Tail,
                },
                State::Tail => match current {
                    UseTree::Name(item) => {
                        // Get the segment's ident as a string and intern it and push to the current
                        // path
                        let seg = item.ident.to_string();
                        let seg = interner.intern(seg);
                        path.segments.push(seg);

                        // Construct a use entry
                        let val = Import {
                            concrete: path,
                            public,
                        };

                        // Push to the list
                        if into.insert(seg, val).is_some() {
                            return Err(GeneratorError::MultipleObjectsWithSameName);
                        }

                        // This is a terminal state so exit the function
                        return Ok(());
                    }
                    UseTree::Rename(item) => {
                        // Get the segment's ident as a string and intern it and push to the current
                        // path
                        let seg = item.ident.to_string();
                        let seg = interner.intern(seg);
                        path.segments.push(seg);

                        // Get the ident for the rename and intern it
                        let rename = item.rename.to_string();
                        let rename = interner.intern(rename);

                        // Construct a use entry
                        let val = Import {
                            concrete: path,
                            public,
                        };

                        // Push to the list
                        if into.insert(rename, val).is_some() {
                            return Err(GeneratorError::MultipleObjectsWithSameName);
                        }

                        // This is a terminal state so exit the function
                        return Ok(());
                    }
                    UseTree::Glob(_) => {
                        // Get the segment's ident as a string and intern it and push to the current
                        // path
                        let seg = interner.intern("*");
                        path.segments.push(seg);

                        // Construct a use entry
                        let val = Import {
                            concrete: path,
                            public,
                        };

                        // Push to the list
                        if into.insert(seg, val).is_some() {
                            return Err(GeneratorError::MultipleObjectsWithSameName);
                        }

                        // This is a terminal state so exit the function
                        return Ok(());
                    }
                    UseTree::Group(item) => {
                        for v in item.items.iter() {
                            Self::resolve_use_into(interner, into, Some(path.clone()), public, v)?;
                        }
                        return Ok(());
                    }
                    UseTree::Path(_) => unreachable!(),
                },
            }
        }
    }
}

///
/// An internal function for cloning the namespace stack that handles an edge case where we need to
/// discard the first element when the len > 0
///
fn clone_namespace_stack(stack: &[StrId]) -> Vec<StrId> {
    if stack.is_empty() {
        stack.to_vec()
    } else {
        stack[1..].to_vec()
    }
}
