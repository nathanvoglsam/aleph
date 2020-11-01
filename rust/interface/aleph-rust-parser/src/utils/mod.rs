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

use crate::ast::Path;
use crate::interner::{Interner, StrId};

/// Takes a `syn::Path` and produces a flattened string with '.' as the segment separator
pub fn path_to_string(path: &syn::Path) -> String {
    // Convert rust's path into a flat string with '.' as a separator
    let mut our_path = String::new();
    path.segments.pairs().for_each(|v| {
        let (segment, token) = v.into_tuple();
        our_path.push_str(&segment.ident.to_string());
        if token.is_some() {
            our_path.push('.');
        }
    });
    our_path
}

/// Internal function for drilling through an arbitrary level of `syn::Type::Paren` wrapping
pub fn drill_through_parens(ty: &syn::Type) -> &syn::Type {
    // Trivial to do iteratively, so do it iteratively
    let mut ty = ty;
    while let syn::Type::Paren(t) = ty {
        ty = t.elem.as_ref();
    }
    ty
}

///
/// An internal function for cloning the namespace stack that handles an edge case where we need to
/// discard the first element when the len > 0
///
pub fn clone_namespace_stack(stack: &[StrId]) -> Vec<StrId> {
    if stack.is_empty() {
        stack.to_vec()
    } else {
        stack[1..].to_vec()
    }
}

pub fn relative_to_absolute_path(
    interner: &mut Interner,
    name_stack: &[StrId],
    path: &mut Path,
) -> Option<()> {
    // Intern some identifiers that we'll need
    let crate_ident = interner.intern("crate");
    let super_ident = interner.intern("super");
    let self_ident = interner.intern("self");

    // The first segment will let us decide where the path intends to begin
    // resolving from
    let first = *path.segments.first().unwrap();
    if first == super_ident {
        if name_stack.is_empty() {
            None
        } else {
            // "super" means to begin resolving from the parent module so substitute the
            // old path with an absolute path that resolves from the crate root
            let mut new_path = name_stack.to_vec();
            new_path.pop();
            new_path.extend_from_slice(&path.segments[1..]);
            path.segments = new_path;
            path.absolute = true;
            Some(())
        }
    } else if first == self_ident {
        // "self" in a use statement means to resolve from within the module itself
        // so replace the old "self" path with an absolute path
        let mut new_path = name_stack.to_vec();
        new_path.extend_from_slice(&path.segments[1..]);
        path.segments = new_path;
        path.absolute = true;
        Some(())
    } else if first == crate_ident {
        // The only other special path segment is "crate" which means to start from
        // the crate root. We handle absolute paths differently in our internal path
        // type so we remove the "crate" segment and mark as absolute
        path.segments = path.segments[1..].to_vec();
        path.absolute = true;
        Some(())
    } else {
        let mut new_path = name_stack.to_vec();
        new_path.extend_from_slice(&path.segments);
        path.segments = new_path;
        path.absolute = true;
        Some(())
    }
}
