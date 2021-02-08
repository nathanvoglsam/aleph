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
use crate::ast::{Class, Import, Interface, Path};
use crate::interner::{Interner, StrId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

enum ModuleItemInternal<'a> {
    Class(&'a RefCell<Class>),
    Module(*const Module),
    Interface(&'a RefCell<Interface>),
    Import((Path, &'a RefCell<Import>)),
}

enum ModuleObjectInternal<'a> {
    Class(&'a RefCell<Class>),
    Module(*const Module),
    Interface(&'a RefCell<Interface>),
}

/// Represents the set of types of ModuleItem
pub enum ModuleItemType {
    Class,
    Module,
    Interface,
    Import,
}

/// Represents the union of a class, module or use import. Primarily used as a function return type
pub enum ModuleItem<'a> {
    Class(std::cell::Ref<'a, Class>),
    Module(&'a Module),
    Interface(std::cell::Ref<'a, Interface>),
    Import((Path, std::cell::Ref<'a, Import>)),
}

/// Represents the union of a class, module or use import. Primarily used as a function return type
pub enum ModuleItemMut<'a> {
    Class(std::cell::RefMut<'a, Class>),
    Module(&'a mut Module),
    Interface(std::cell::RefMut<'a, Interface>),
    Import((Path, std::cell::RefMut<'a, Import>)),
}

/// Represents the set of types of ModuleItem
pub enum ModuleObjectType {
    Class,
    Module,
    Interface,
}

/// Represents the union of a class and module. Primarily used as a function return type
pub enum ModuleObject<'a> {
    Class(std::cell::Ref<'a, Class>),
    Module(&'a Module),
    Interface(std::cell::Ref<'a, Interface>),
}

/// Represents the union of a class and module. Primarily used as a function return type
pub enum ModuleObjectMut<'a> {
    Class(std::cell::RefMut<'a, Class>),
    Module(&'a mut Module),
    Interface(std::cell::RefMut<'a, Interface>),
}

/// Internal struct for representing a module graph
#[derive(Clone, Debug, Default)]
pub struct Module {
    /// The list of named entities in scope in this module from use statements
    pub imports: HashMap<StrId, RefCell<Import>>,

    /// The set of structs in this module
    pub classes: HashMap<StrId, RefCell<Class>>,

    /// The set of interfaces defined in this module
    pub interfaces: HashMap<StrId, RefCell<Interface>>,

    /// A map of submodules
    pub sub_modules: HashMap<StrId, Module>,
}

// Functions that implement behavior on a completely constructed module
impl Module {
    /// Internal function for handling the nearly identical implementation of an immutable and
    /// mutable getter.
    ///
    /// Uses the given path to attempt to look up the item that the path refers to.
    ///
    /// An item can refer to an interface, class, module or an import.
    ///
    /// If the path is trying to lookup an object through an import then this function will
    /// terminate before fully resolving the path through the import and will instead return the
    /// import statement itself.
    ///
    /// Further logic needs to be implemented on top of this function to fully resolve a path down
    /// to an object (interface, class or module)
    ///
    #[inline(always)]
    fn lookup_internal(&self, path: &Path) -> Option<ModuleItemInternal> {
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
        let mut out: Option<ModuleItemInternal> = None;
        for seg in path.segments.iter() {
            return match state {
                State::Module(i) => {
                    if let Some(import) = i.imports.get(seg) {
                        let import_ref = import.borrow();
                        let use_path = if import_ref.concrete.absolute {
                            import_ref.concrete.clone()
                        } else {
                            let mut segments = path.segments[0..depth].to_vec();
                            segments.extend_from_slice(&import_ref.concrete.segments);
                            Path::new(segments, true)
                        };
                        state = State::Terminal;
                        out = Some(ModuleItemInternal::Import((use_path, import)));
                        depth += 1;
                        continue;
                    }
                    if let Some(module) = i.sub_modules.get(seg) {
                        state = State::Module(module);
                        out = Some(ModuleItemInternal::Module(module as *const Module));
                        depth += 1;
                        continue;
                    }
                    if let Some(class) = i.classes.get(seg) {
                        state = State::Terminal;
                        out = Some(ModuleItemInternal::Class(class));
                        depth += 1;
                        continue;
                    }
                    if let Some(interface) = i.interfaces.get(seg) {
                        state = State::Terminal;
                        out = Some(ModuleItemInternal::Interface(interface));
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

    /// This function implements further logic on top of the `Self::lookup_internal` function to
    /// fully resolve a path down to a concrete object (interface, class or module)
    ///
    /// # Warning
    ///
    /// This only works if all `use` imports that import objects from external crates have been
    /// stripped from the module graph
    ///
    #[inline(always)]
    fn lookup_object_internal(&self, path: &Path) -> Option<(Path, ModuleObjectInternal)> {
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
        while let Some(item) = self.lookup_internal(&*current) {
            match item {
                ModuleItemInternal::Class(i) => {
                    let path = match current {
                        State::Owned(i) => i,
                        State::Ref(i) => i.clone(),
                    };
                    return Some((path, ModuleObjectInternal::Class(i)));
                }
                ModuleItemInternal::Module(i) => {
                    let path = match current {
                        State::Owned(i) => i,
                        State::Ref(i) => i.clone(),
                    };
                    return Some((path, ModuleObjectInternal::Module(i)));
                }
                ModuleItemInternal::Interface(i) => {
                    let path = match current {
                        State::Owned(i) => i,
                        State::Ref(i) => i.clone(),
                    };
                    return Some((path, ModuleObjectInternal::Interface(i)));
                }
                ModuleItemInternal::Import((i, _)) => current = State::Owned(i),
            }
        }
        None
    }

    /// Returns the type of item the path refers to if it exists, or `None` if the path does not
    /// refer to anything
    pub fn item_exists(&self, path: &Path) -> Option<ModuleItemType> {
        match self.lookup_internal(path) {
            None => None,
            Some(i) => match i {
                ModuleItemInternal::Class(_) => Some(ModuleItemType::Class),
                ModuleItemInternal::Module(_) => Some(ModuleItemType::Module),
                ModuleItemInternal::Interface(_) => Some(ModuleItemType::Interface),
                ModuleItemInternal::Import(_) => Some(ModuleItemType::Import),
            },
        }
    }

    /// Get a reference to the item path refers to if it exists
    pub fn lookup(&self, path: &Path) -> Option<ModuleItem> {
        match self.lookup_internal(path) {
            None => None,
            Some(object) => match object {
                ModuleItemInternal::Class(i) => Some(ModuleItem::Class(i.borrow())),
                ModuleItemInternal::Module(i) => {
                    let i = unsafe { &*i };
                    Some(ModuleItem::Module(i))
                }
                ModuleItemInternal::Interface(i) => Some(ModuleItem::Interface(i.borrow())),
                ModuleItemInternal::Import((path, i)) => {
                    Some(ModuleItem::Import((path, i.borrow())))
                }
            },
        }
    }

    /// Get a mutable reference to the item path refers to if it exists
    pub fn lookup_mut(&mut self, path: &Path) -> Option<ModuleItemMut> {
        match self.lookup_internal(path) {
            None => None,
            Some(object) => match object {
                ModuleItemInternal::Class(i) => Some(ModuleItemMut::Class(i.borrow_mut())),
                ModuleItemInternal::Module(i) => {
                    let i = i as *mut Module;
                    let i = unsafe { &mut *i };
                    Some(ModuleItemMut::Module(i))
                }
                ModuleItemInternal::Interface(i) => Some(ModuleItemMut::Interface(i.borrow_mut())),
                ModuleItemInternal::Import((path, i)) => {
                    Some(ModuleItemMut::Import((path, i.borrow_mut())))
                }
            },
        }
    }

    /// Returns the type of object path refers to, or `None` if the object doesn't refer to anything
    pub fn object_exists(&self, path: &Path) -> Option<ModuleObjectType> {
        match self.lookup_object_internal(path) {
            None => None,
            Some((_, i)) => match i {
                ModuleObjectInternal::Class(_) => Some(ModuleObjectType::Class),
                ModuleObjectInternal::Module(_) => Some(ModuleObjectType::Module),
                ModuleObjectInternal::Interface(_) => Some(ModuleObjectType::Interface),
            },
        }
    }

    /// Returns a reference to the object path refers to, or `None` if the path doesn't refer to
    /// any object.
    ///
    /// # Warning
    ///
    /// This only works if all `use` imports that import objects from external crates have been
    /// stripped from the module graph
    ///
    pub fn lookup_object(&self, path: &Path) -> Option<(Path, ModuleObject)> {
        match self.lookup_object_internal(path) {
            None => None,
            Some((path, i)) => {
                let out = match i {
                    ModuleObjectInternal::Class(i) => ModuleObject::Class(i.borrow()),
                    ModuleObjectInternal::Module(i) => {
                        let i = unsafe { &*i };
                        ModuleObject::Module(i)
                    }
                    ModuleObjectInternal::Interface(i) => ModuleObject::Interface(i.borrow()),
                };
                Some((path, out))
            }
        }
    }

    /// Returns a mutable reference to the object path refers to, or `None` if the path doesn't
    /// refer to any object.
    /// # Warning
    ///
    /// This only works if all `use` imports that import objects from external crates have been
    /// stripped from the module graph
    ///
    pub fn lookup_object_mut(&mut self, path: &Path) -> Option<(Path, ModuleObjectMut)> {
        match self.lookup_object_internal(path) {
            None => None,
            Some((path, i)) => {
                let out = match i {
                    ModuleObjectInternal::Class(i) => ModuleObjectMut::Class(i.borrow_mut()),
                    ModuleObjectInternal::Module(i) => {
                        let i = i as *mut Module;
                        let i = unsafe { &mut *i };
                        ModuleObjectMut::Module(i)
                    }
                    ModuleObjectInternal::Interface(i) => {
                        ModuleObjectMut::Interface(i.borrow_mut())
                    }
                };
                Some((path, out))
            }
        }
    }

    /// Internal function used for debug printing the AST with full names and not StrId indexes
    #[allow(unused)]
    pub(crate) fn debug_print(&self, interner: &Interner) -> Option<()> {
        let crate_ident = interner.contains("crate")?;
        let mut name_stack = vec![];
        let mut iter_stack = vec![IterUnion::Root(Some((crate_ident, self)))];
        'outer: while let Some(mut iter) = iter_stack.pop() {
            while let Some((module_name, module)) = iter.next() {
                let indent = {
                    let mut indent = String::new();
                    for _ in 0..name_stack.len() {
                        indent.push_str("|   ");
                    }
                    indent
                };

                print!("{}module ", &indent);
                name_stack
                    .iter()
                    .for_each(|v| print!("{}::", interner.lookup(*v)));
                print!("{}\n", interner.lookup(module_name));

                for (u_name, u) in module.imports.iter() {
                    let u = u.borrow();
                    let vis = if u.public { "public" } else { "" };
                    let u_name_str = interner.lookup(*u_name);
                    let u_path_str = u.concrete.to_string(interner);
                    println!(
                        "{}|   use {} as {} {}",
                        &indent, &u_path_str, u_name_str, vis
                    );
                }

                for (class_name, class) in module.classes.iter() {
                    let class = class.borrow();
                    let tag = if class.public {
                        "public class"
                    } else {
                        "class"
                    };
                    println!("{}|   {} {}", &indent, tag, interner.lookup(*class_name));
                    for (field_name, field) in class.fields.iter() {
                        let field_name = interner.lookup(*field_name);
                        let field_type = field.to_string(interner);
                        println!("{}|   |   field {}: {}", &indent, field_name, &field_type);
                    }
                    for (function_name, function) in class.functions.iter() {
                        let function_name = interner.lookup(*function_name);
                        print!("{}|   |   fn {}(", &indent, function_name);
                        for arg in function.args.iter() {
                            let arg_str = arg.to_string(interner);
                            print!("{},", arg_str);
                        }
                        println!(") -> {}", function.returns.to_string(interner));
                    }
                    for implements in class.implements.iter() {
                        let string = implements.to_string(interner);
                        println!("|   |   implements {}", string);
                    }
                }

                for (interface_name, interface) in module.interfaces.iter() {
                    let interface = interface.borrow();
                    let tag = if interface.public {
                        "public interface"
                    } else {
                        "interface"
                    };
                    println!(
                        "{}|   {} {}",
                        &indent,
                        tag,
                        interner.lookup(*interface_name)
                    );
                    for (function_name, function) in interface.functions.iter() {
                        let function_name = interner.lookup(*function_name);
                        print!("{}|   |   fn {}(", &indent, function_name);
                        for arg in function.args.iter() {
                            let arg_str = arg.to_string(interner);
                            print!("{},", arg_str);
                        }
                        println!(") -> {}", function.returns.to_string(interner));
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
