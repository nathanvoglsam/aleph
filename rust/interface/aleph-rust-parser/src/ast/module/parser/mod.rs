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
use crate::ast::module::{ModuleItemMut, ModuleObject, ModuleObjectMut};
use crate::ast::{Class, Function, GeneratorError, Import, Interface, Module, Path, Result, Type};
use crate::interner::{Interner, StrId};
use crate::utils::{clone_namespace_stack, relative_to_absolute_path};
use std::collections::HashMap;
use syn::export::Span;

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
        let root = module_stack.pop().unwrap(); // Must exist

        // Validate that there are no name collisions between structs and imported objects
        let root = Self::use_collision_validation_stage(root)?;

        // Pass back over the module tree and resolve all use statements to a fully qualified path,
        // prune imports from external crates and enforce that all uses refer to a valid object
        let root = Self::use_fixup_stage(root, interner)?;

        // Iterate over the file a second time. We now know all of the structs we need to generate
        // interfaces for so we can look for impl blocks that associated functions with the structs
        item_iter_stack.push(file.items.iter());
        let root =
            Self::impl_resolve_stage(root, interner, &mut namespace_stack, &mut item_iter_stack)?;

        // Iterate over the module again to resolve all paths to be absolute
        let root = Self::struct_path_resolve_stage(root, interner)?;

        // Now that everything is an absolute path we can check that every path trying to refer to
        // a type, actually refers to a type
        let root = Self::struct_path_check_stage(root, interner)?;

        Ok(root)
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
                        let has_attr = item.attrs.iter().any(|attr| &attr.path == &aleph_interface);
                        let has_generics =
                            item.generics.gt_token.is_some() || item.generics.lt_token.is_some();

                        // Defining the attribute on a struct with generic parameters is an error as
                        // there is no sane way to represent generics across an FFI boundary
                        if has_attr && has_generics {
                            return Err(GeneratorError::AttributeOnGenericObject);
                        }

                        // We only need to check for the attribute here as we've already guarded
                        // from the `has_generics` case by exiting in the above block
                        if has_attr {
                            let public = Self::resolve_visibility(&item.vis)?;
                            let struct_name = item.ident.to_string();
                            let struct_name = interner.intern(struct_name);

                            let mut class = Class {
                                fields: Default::default(),
                                functions: Default::default(),
                                implements: Default::default(),
                                public,
                            };

                            for field in item.fields.iter() {
                                let field_name = field.ident.as_ref().unwrap().to_string();
                                let field_name = interner.intern(field_name);

                                let ty = if let Some(ty) =
                                    Type::from_struct_field(interner, &field.ty)
                                {
                                    ty
                                } else {
                                    return Err(GeneratorError::UnsupportedStructField);
                                };

                                class.fields.insert(field_name, ty);
                            }

                            internal_module.classes.insert(struct_name, class);
                        }
                    }
                    syn::Item::Trait(item) => {
                        let has_attr = item.attrs.iter().any(|attr| &attr.path == &aleph_interface);
                        let has_generics =
                            item.generics.gt_token.is_some() || item.generics.lt_token.is_some();

                        // Defining the attribute on a struct with generic parameters is an error as
                        // there is no sane way to represent generics across an FFI boundary
                        if has_attr && has_generics {
                            return Err(GeneratorError::AttributeOnGenericObject);
                        }

                        if has_attr {
                            let public = Self::resolve_visibility(&item.vis)?;
                            let interface_name = item.ident.to_string();
                            let interface_name = interner.intern(interface_name);

                            let mut interface = Interface {
                                functions: Default::default(),
                                public,
                            };

                            for trait_item in item.items.iter() {
                                if let syn::TraitItem::Method(m) = trait_item {
                                    let f_name = m.sig.ident.to_string();
                                    let f_name = interner.intern(f_name);
                                    let f = Function::from_function_signature(interner, &m.sig)
                                        .ok_or(GeneratorError::UnsupportedMethodSignature)?;
                                    interface.functions.insert(f_name, f);
                                }
                            }

                            internal_module.interfaces.insert(interface_name, interface);
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

    fn use_collision_validation_stage(root: Module) -> Result<Module> {
        let mut stack = Vec::new();
        stack.push(&root);
        while let Some(module) = stack.pop() {
            // Make sure no `uses` import colliding names
            for (name, _) in module.imports.iter() {
                if module.classes.contains_key(name) {
                    return Err(GeneratorError::MultipleObjectsWithSameName);
                }
                if module.sub_modules.contains_key(name) {
                    return Err(GeneratorError::MultipleObjectsWithSameName);
                }
            }

            // Make sure no module name collides with a struct name
            for (name, sub_module) in module.sub_modules.iter() {
                if module.classes.contains_key(name) {
                    return Err(GeneratorError::MultipleObjectsWithSameName);
                }
                stack.push(sub_module);
            }
        }
        Ok(root)
    }

    fn use_fixup_stage(root: Module, interner: &mut Interner) -> Result<Module> {
        // The first pass resolves all paths to be fully qualified but may not actually be valid
        // paths to an object (module or struct)
        let root = Self::use_fixup_stage_pass_1(root, interner)?;

        // The second pass will validate that all use statements that refer to objects within the
        // crate refer to an object. This should mean any path not prefixed with crate:: must refer
        // to an external crate and can be ignored
        let root = Self::use_fixup_stage_pass_2(root, interner)?;

        Ok(root)
    }

    fn use_fixup_stage_pass_1(mut root: Module, interner: &mut Interner) -> Result<Module> {
        // Intern some identifiers that we'll need
        let crate_ident = interner.intern("crate");
        let super_ident = interner.intern("super");
        let self_ident = interner.intern("self");

        // Set up our on heap stack for our depth first traversal
        let mut name_stack = vec![];
        let mut iter_stack: Vec<IterUnionMut> =
            vec![IterUnionMut::Root(Some((crate_ident, &mut root)))];

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

        Ok(root)
    }

    fn use_fixup_stage_pass_2(mut root: Module, interner: &mut Interner) -> Result<Module> {
        // Intern some identifiers that we'll need
        let crate_ident = interner.intern("crate");

        // Set up our on heap stack for our depth first traversal
        let mut name_stack = vec![];
        let mut iter_stack: Vec<IterUnion> = vec![IterUnion::Root(Some((crate_ident, &root)))];

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

        Ok(root)
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
            syn::UseTree::Path(_) => State::Path,
            syn::UseTree::Name(_) | syn::UseTree::Rename(_) | syn::UseTree::Glob(_) => {
                if path.segments.is_empty() {
                    return Err(GeneratorError::InvalidUseSyntax);
                } else {
                    State::Tail
                }
            }
            syn::UseTree::Group(_) => State::Tail,
        };

        // This is a recursive function flattened into a loop and is a hand written parser state
        // machine so may look a little chaotic
        loop {
            match state {
                State::Path => match current {
                    // If we've got another path push it's segment onto the path we're currently
                    // building and update the `current` to point to the next item in the chain
                    syn::UseTree::Path(i) => {
                        // Get the segment's ident as a string and intern it and push to the current
                        // path
                        let seg = i.ident.to_string();
                        let seg = interner.intern(seg);
                        path.segments.push(seg);

                        // Step to the next node in the use tree
                        current = &i.tree;
                    }
                    syn::UseTree::Name(_) => state = State::Tail,
                    syn::UseTree::Rename(_) => state = State::Tail,
                    syn::UseTree::Glob(_) => state = State::Tail,
                    syn::UseTree::Group(_) => state = State::Tail,
                },
                State::Tail => match current {
                    syn::UseTree::Name(item) => {
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
                    syn::UseTree::Rename(item) => {
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
                    syn::UseTree::Glob(_) => {
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
                    syn::UseTree::Group(item) => {
                        for v in item.items.iter() {
                            Self::resolve_use_into(interner, into, Some(path.clone()), public, v)?;
                        }
                        return Ok(());
                    }
                    syn::UseTree::Path(_) => unreachable!(),
                },
            }
        }
    }

    fn impl_resolve_stage(
        mut root: Module,
        interner: &mut Interner,
        namespace_stack: &mut Vec<StrId>,
        item_iter_stack: &mut Vec<std::slice::Iter<syn::Item>>,
    ) -> Result<Module> {
        'outer: while let Some(mut items) = item_iter_stack.pop() {
            while let Some(item) = items.next() {
                match item {
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
                            continue 'outer;
                        }
                    }
                    syn::Item::Impl(item) => match &item.self_ty.as_ref() {
                        syn::Type::Path(path) => {
                            Self::handle_impl_on_path(
                                interner,
                                namespace_stack,
                                &mut root,
                                &item,
                                path,
                            )?;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            namespace_stack.pop();
        }
        Ok(root)
    }

    fn handle_impl_on_path(
        interner: &mut Interner,
        namespace_stack: &mut Vec<StrId>,
        module: &mut Module,
        item: &syn::ItemImpl,
        path: &syn::TypePath,
    ) -> Result<()> {
        // Intern the path and resolve to an absolute path
        let mut path = Path::from_syn(interner, &path.path);
        relative_to_absolute_path(interner, namespace_stack, &mut path)
            .ok_or(GeneratorError::InvalidUsePath)?;

        // Now we need to check if this impl block is actually implementing functions on a struct
        // that we should be generating an interface for. We already have every struct we should
        // care about in the module, so we just look up the object with the absolute path we
        // created. If it exists in the module then we should parse the impl block. We also need to
        // make sure the impl block is defined on a struct and not something else so we also emit
        // an error for this case
        let object = module.lookup_object_mut(&path);
        if let Some((_, ModuleObjectMut::Class(class))) = object {
            // Now that we know we should be parsing this impl block we can check some error cases
            // that would only be errors for impl blocks on interface structs. Here we're checking
            // if we've got any generics in the impl signature, which we can't properly handle right
            // now.
            let has_generics = item.generics.lt_token.is_some() || item.generics.gt_token.is_some();
            if has_generics {
                return Err(GeneratorError::AttributeOnGenericObject);
            }

            if let Some((negative_token, trait_path, _)) = &item.trait_ {
                // We don't care about negative trait bounds as there's no information useful to
                // generating the FFI interface in them
                if negative_token.is_some() {
                    return Ok(());
                }

                let mut trait_path = Path::from_syn(interner, trait_path);
                relative_to_absolute_path(interner, namespace_stack, &mut trait_path)
                    .ok_or(GeneratorError::InvalidUsePath)?;
                class.implements.push(trait_path);
            } else {
                // Iterate over all items in the impl block
                for item in item.items.iter() {
                    // And select only the function/method declarations as those are the only ones
                    // we care about
                    if let syn::ImplItem::Method(method) = item {
                        // Attempt to parse the function signature and add it to the class's list of
                        // functions
                        let function = Function::from_function_signature(interner, &method.sig)
                            .ok_or(GeneratorError::UnsupportedMethodSignature)?;
                        let name = method.sig.ident.to_string();
                        let name = interner.intern(name);
                        class.functions.insert(name, function);
                    }
                }
            }
        }
        Ok(())
    }

    fn struct_path_resolve_stage(mut root: Module, interner: &mut Interner) -> Result<Module> {
        // Intern some identifiers that we'll need
        let crate_ident = interner.intern("crate");

        // SAFETY:
        // We own the `root` module so we, by definition, must be the only place in the code that
        // has a reference to this object.
        //
        // Based on this it means this function holds exclusive access to the root module. Due to
        // the nature of what we're doing we need to check for the existence of an object within
        // the module graph itself while mutably iterating the module graph. The borrow checker
        // does not like this.
        //
        // The module graph's structure (links) is never modified so technically we aren't actually
        // mutable borrowing the graph, only some of the contents of the graph. Which is something
        // the borrow checker can't really represent in a sane way and something we can't work
        // around in safe code without riddling the code with RefCell.
        //
        // For now I will use a super unsafe workaround that, to my knowledge, is still safe as the
        // single use of the aliased data only reads data that isn't written to anyway so even if
        // the compiler does some funky optimizations it can't break what we're doing anyway.
        //
        // Yes I know undefined behaviour means the compiler can do literally anything it wants, but
        // there's nothing sane it could do to break this so I'm going to stick with it for now
        //
        // I might try the RefCell solution but that's going to make the code *way* uglier
        let root_unsafe: &Module = unsafe {
            let root_unsafe: *const Module = core::mem::transmute(&root);
            &*root_unsafe
        };

        // Set up our on heap stack for our depth first traversal
        let mut name_stack = vec![];
        let mut iter_stack: Vec<IterUnionMut> =
            vec![IterUnionMut::Root(Some((crate_ident, &mut root)))];

        'outer: while let Some(mut iter) = iter_stack.pop() {
            while let Some((module_name, module)) = iter.next() {
                name_stack.push(module_name);
                for (_, class) in module.classes.iter_mut() {
                    for (_, field) in class.fields.iter_mut() {
                        field.relative_to_absolute_path(&name_stack[1..], interner)?;
                    }
                    for (_, func) in class.functions.iter_mut() {
                        func.relative_to_absolute_path(&name_stack[1..], interner)?;
                    }
                    class.implements.retain(|v| {
                        let object = root_unsafe.lookup_object(v);
                        if let Some((_, ModuleObject::Interface(_))) = object {
                            true
                        } else {
                            false
                        }
                    });
                }
                for (_, interface) in module.interfaces.iter_mut() {
                    for (_, func) in interface.functions.iter_mut() {
                        func.relative_to_absolute_path(&name_stack[1..], interner)?;
                    }
                }

                iter_stack.push(iter);
                iter_stack.push(IterUnionMut::Map(module.sub_modules.iter_mut()));
                continue 'outer;
            }
            name_stack.pop();
        }

        Ok(root)
    }

    fn struct_path_check_stage(root: Module, interner: &mut Interner) -> Result<Module> {
        // Intern some identifiers that we'll need
        let crate_ident = interner.intern("crate");

        // Set up our on heap stack for our depth first traversal
        let mut name_stack = vec![];
        let mut iter_stack: Vec<IterUnion> = vec![IterUnion::Root(Some((crate_ident, &root)))];

        'outer: while let Some(mut iter) = iter_stack.pop() {
            while let Some((module_name, module)) = iter.next() {
                name_stack.push(module_name);
                for (_, class) in module.classes.iter() {
                    for (_, field) in class.fields.iter() {
                        field.check_path_exists_as_class_in_module(&root)?;
                    }
                    for (_, func) in class.functions.iter() {
                        func.check_path_exists_as_class_in_module(&root)?;
                    }
                }
                for (_, interface) in module.interfaces.iter() {
                    for (_, func) in interface.functions.iter() {
                        func.check_path_exists_as_class_in_module(&root)?;
                    }
                }

                iter_stack.push(iter);
                iter_stack.push(IterUnion::Map(module.sub_modules.iter()));
                continue 'outer;
            }
            name_stack.pop();
        }

        Ok(root)
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
            syn::Visibility::Public(_) => Ok(true),
            syn::Visibility::Crate(_) => unimplemented!(),
            syn::Visibility::Restricted(i) => {
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
            syn::Visibility::Inherited => Ok(false),
        }
    }
}
