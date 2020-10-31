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

mod iter;
mod parser;

use crate::ast::module::iter::IterUnion;
use crate::ast::{Class, Import, Path};
use crate::interner::{Interner, StrId};
use std::collections::HashMap;
use std::ops::Deref;

/// Represents the union of a class, module or use import. Primarily used as a function return type
pub enum ModuleItem<'a> {
    Class(&'a Class),
    Module(&'a Module),
    Import((Path, &'a Import)),
}

/// Represents the union of a class, module or use import. Primarily used as a function return type
pub enum ModuleItemMut<'a> {
    Class(&'a mut Class),
    Module(&'a mut Module),
    Import((Path, &'a mut Import)),
}

/// Represents the union of a class and module. Primarily used as a function return type
pub enum ModuleObject<'a> {
    Class(&'a Class),
    Module(&'a Module),
}

/// Represents the union of a class and module. Primarily used as a function return type
pub enum ModuleObjectMut<'a> {
    Class(&'a mut Class),
    Module(&'a mut Module),
}

/// Internal struct for representing a module graph
#[derive(Clone, Debug, Default)]
pub struct Module {
    /// The list of named entities in scope in this module from use statements
    pub imports: HashMap<StrId, Import>,

    //pub uses2: HashMap<StrId, Use>,
    /// The set of structs in this module
    pub structs: HashMap<StrId, Class>,

    /// A map of submodules
    pub sub_modules: HashMap<StrId, Module>,
}

// Functions that implement behavior on a completely constructed module
impl Module {
    pub fn lookup(&self, path: &Path) -> Option<ModuleItem> {
        enum State<'a> {
            Module(&'a Module),
            Terminal,
        }

        // Path must be absolute to lookup on a module
        if !path.absolute {
            return None;
        }

        let mut depth = 0;
        let mut state = State::Module(self);
        let mut out: Option<ModuleItem> = None;
        for seg in path.segments.iter() {
            return match state {
                State::Module(i) => {
                    if let Some(u) = i.imports.get(seg) {
                        let use_path = if u.concrete.absolute {
                            u.concrete.clone()
                        } else {
                            let mut segments = path.segments[0..depth].to_vec();
                            segments.extend_from_slice(&u.concrete.segments);
                            Path::new(segments, true)
                        };
                        state = State::Terminal;
                        out = Some(ModuleItem::Import((use_path, u)));
                        depth += 1;
                        continue;
                    }
                    if let Some(module) = i.sub_modules.get(seg) {
                        state = State::Module(module);
                        out = Some(ModuleItem::Module(module));
                        depth += 1;
                        continue;
                    }
                    if let Some(class) = i.structs.get(seg) {
                        state = State::Terminal;
                        out = Some(ModuleItem::Class(class));
                        depth += 1;
                        continue;
                    }
                    None
                }
                State::Terminal => None,
            };
        }

        out
    }

    pub fn lookup_mut(&mut self, path: &Path) -> Option<ModuleItemMut> {
        self.lookup(path).map(|v| {
            // Safety:
            // Because I don't want to duplicate literally the same code just to look up a mutable
            // reference to an object I use this unsafe solution to cast a shared reference to a
            // mutable reference. The interface remains safe to use as the `&mut self` bound will
            // enforce that the whole object is mutably borrowed with a call to this function so we
            // can't give out aliasing mutable references with this interface
            #[allow(mutable_transmutes)]
            unsafe {
                match v {
                    ModuleItem::Class(i) => ModuleItemMut::Class(core::mem::transmute(i)),
                    ModuleItem::Module(i) => ModuleItemMut::Module(core::mem::transmute(i)),
                    ModuleItem::Import(i) => ModuleItemMut::Import(core::mem::transmute(i)),
                }
            }
        })
    }

    ///
    ///
    /// # Warning
    ///
    /// This only works if all `use` imports that import objects from external crates have been
    /// stripped from the module graph
    ///
    pub fn lookup_object(&self, path: &Path) -> Option<(Path, ModuleObject)> {
        enum State<'a> {
            Owned(Path),
            Ref(&'a Path),
        }
        impl<'a> Deref for State<'a> {
            type Target = Path;

            fn deref(&self) -> &Self::Target {
                match self {
                    State::Owned(i) => i,
                    State::Ref(i) => i,
                }
            }
        }

        let mut current = State::Ref(path);
        while let Some(item) = self.lookup(&*current) {
            match item {
                ModuleItem::Class(i) => {
                    let path = match current {
                        State::Owned(i) => i,
                        State::Ref(i) => i.clone(),
                    };
                    return Some((path, ModuleObject::Class(i)));
                }
                ModuleItem::Module(i) => {
                    let path = match current {
                        State::Owned(i) => i,
                        State::Ref(i) => i.clone(),
                    };
                    return Some((path, ModuleObject::Module(i)));
                }
                ModuleItem::Import((i, _)) => current = State::Owned(i),
            }
        }
        None
    }

    ///
    ///
    /// # Warning
    ///
    /// This only works if all `use` imports that import objects from external crates have been
    /// stripped from the module graph
    ///
    pub fn lookup_object_mut(&mut self, path: &Path) -> Option<(Path, ModuleObjectMut)> {
        self.lookup_object(path).map(|(path, object)| {
            // Safety:
            // Because I don't want to duplicate literally the same code just to look up a mutable
            // reference to an object I use this unsafe solution to cast a shared reference to a
            // mutable reference. The interface remains safe to use as the `&mut self` bound will
            // enforce that the whole object is mutably borrowed with a call to this function so we
            // can't give out aliasing mutable references with this interface
            #[allow(mutable_transmutes)]
            let object = unsafe {
                match object {
                    ModuleObject::Class(i) => ModuleObjectMut::Class(core::mem::transmute(i)),
                    ModuleObject::Module(i) => ModuleObjectMut::Module(core::mem::transmute(i)),
                }
            };
            (path, object)
        })
    }

    /// Internal function used for debug printing the AST with full names and not StrId indexes
    pub(crate) fn debug_print(&self, interner: &Interner) -> Option<()> {
        let crate_ident = interner.contains("crate")?;
        let mut name_stack = vec![];
        let mut iter_stack = vec![IterUnion::Root(Some((crate_ident, self)))];
        'outer: while let Some(mut iter) = iter_stack.pop() {
            while let Some((module_name, module)) = iter.next() {
                let indent = {
                    let mut indent = String::new();
                    for _ in 0..name_stack.len() {
                        indent.push_str("  ");
                    }
                    indent
                };

                print!("{}module ", &indent);
                name_stack
                    .iter()
                    .for_each(|v| print!("{}::", interner.lookup(*v)));
                print!("{}\n", interner.lookup(module_name));

                for (u_name, u) in module.imports.iter() {
                    let vis = if u.public { "public" } else { "" };
                    let u_name_str = interner.lookup(*u_name);
                    let u_path_str = u.concrete.to_string(interner);
                    println!("{}  use {} as {} {}", &indent, &u_path_str, u_name_str, vis);
                }

                for (class_name, class) in module.structs.iter() {
                    let tag = if class.public {
                        "public class"
                    } else {
                        "class"
                    };
                    println!("{}  {} {}", &indent, tag, interner.lookup(*class_name));
                    for (field_name, field) in class.inner.fields.iter() {
                        println!("{}    field {}: {:?}", &indent, &field_name, &field);
                    }
                    for (function_name, function) in class.inner.functions.iter() {
                        println!(
                            "{}    function [{}]: {:?}",
                            &indent, &function_name, &function
                        );
                    }
                }

                name_stack.push(module_name);
                iter_stack.push(iter);
                iter_stack.push(IterUnion::Map(module.sub_modules.iter()));
                continue 'outer;
            }
            name_stack.pop();
        }
        Some(())
    }
}
