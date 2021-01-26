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

use proc_macro2::Span;
use syn::{
    Attribute, Block, ForeignItemFn, ItemFn, Signature, Stmt, VisCrate, VisPublic, Visibility,
};

///
/// A builder wrapper for the `syn` crate's `Signature`, `Visibility` and `Vec<Attribute>` structs.
/// These three items are needed for defining a fully qualified function signature, including
/// visibility modifiers and associated attributes.
///
/// This builder is designed to be chained with TODO: rest of docs
///
/// # Warning
///
/// This builder won't validate if the function you try to build is sane. For example, this builder
/// will not complain if you try and make a const unsafe function. A const unsafe function is not
/// valid Rust so will fail to compile, and this builder wont protect you from that.
///
#[derive(Clone)]
pub struct FunctionSignatureBuilder {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
}

impl FunctionSignatureBuilder {
    ///
    /// Creates a new `ItemFnBuilder` for creating a function with the given name.
    ///
    /// # Defaults
    ///
    /// The function the builder will create by default is the least decorated function that can be
    /// written in valid Rust syntax.
    ///
    /// The function can be precisely defined as a function with:
    /// * No visibility specified (no pub, pub(crate), etc)
    /// * No arguments
    /// * No return value
    /// * Is named based on the `name` parameter to this function
    /// * Has no generics
    /// * Is not const
    /// * Is not async
    /// * Is not unsafe
    /// * Is not extern
    /// * Has no attributes
    ///
    /// This is the minimum possible function declaration in Rust and is the most sane default for
    /// expanding on.
    ///
    pub fn new(name: &str) -> Self {
        // Convert name to `Ident` token
        let ident = syn::Ident::new(name, Span::call_site());

        // Default to a function with no attributes
        let attrs = Vec::new();

        // Default to a function with private visibility (i.e, no visibility specifiers)
        let vis = Visibility::Inherited;

        // We need to build a default function signature. This creates the barest possible function.
        // Function with no generics, no return value, non-const, non-async, safe and is non-extern.
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident,
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: Default::default(),
            variadic: None,
            output: syn::ReturnType::Default,
        };

        Self { attrs, vis, sig }
    }

    ///
    /// Consumes the function signature and returns a `syn` `ForeignItemFn` object that defines a
    /// foreign function with the signature defined by this builder.
    ///
    /// A foreign function is one that has no body defined by the rust module, rather it defines a
    /// symbol to import or link from a library file.
    ///
    /// This basically just boils down to a function without a function body.
    ///
    pub fn build_ffi_function(self) -> ForeignItemFn {
        ForeignItemFn {
            attrs: self.attrs,
            vis: self.vis,
            sig: self.sig,
            semi_token: Default::default(),
        }
    }

    ///
    /// Chains this `FunctionSignatureBuilder` into a subsequent `FunctionBuilder` that will build
    /// a function with the signature defined by `self`
    ///
    pub fn build_function(self) -> FunctionBuilder {
        FunctionBuilder::new(self)
    }

    ///
    /// Append the given attribute to the list of attributes associated to the function signature
    ///
    pub fn attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.attrs.push(attribute);
        self
    }

    ///
    /// This function is marked as `const` based on the `constness` parameter.
    ///
    pub fn constness(&mut self, constness: bool) -> &mut Self {
        if constness {
            self.sig.constness = Some(Default::default());
        } else {
            self.sig.constness = None;
        }
        self
    }

    ///
    /// This function is marked as `async` based on the `asyncness` parameter.
    ///
    pub fn asyncness(&mut self, asyncness: bool) -> &mut Self {
        if asyncness {
            self.sig.asyncness = Some(Default::default());
        } else {
            self.sig.asyncness = None;
        }
        self
    }

    ///
    /// This function is marked as `unsafe` based on the `unsafety` parameter.
    ///
    pub fn unsafety(&mut self, unsafety: bool) -> &mut Self {
        if unsafety {
            self.sig.unsafety = Some(Default::default());
        } else {
            self.sig.unsafety = None;
        }
        self
    }

    ///
    /// Adds the `pub` visibility modifier to the function
    ///
    pub fn visibility_pub(&mut self) -> &mut Self {
        // Build the variant
        let vis = VisPublic {
            pub_token: Default::default(),
        };
        let vis = syn::Visibility::Public(vis);

        self.vis = vis;
        self
    }

    ///
    /// Adds the `crate` visibility modifier to the function
    ///
    /// # Warning
    ///
    /// Currently, at the time of writing (latest stable rust being 1.45.2), this modifier is
    /// experimental so will not be valid to use on a stable toolchain.
    ///
    pub fn visibility_crate(&mut self) -> &mut Self {
        // Build the variant
        let vis = VisCrate {
            crate_token: Default::default(),
        };
        let vis = syn::Visibility::Crate(vis);

        self.vis = vis;
        self
    }

    ///
    /// Marks the function's visibility as "Inherited". This means a function with no written
    /// visibility modifier
    ///
    pub fn visibility_inherited(&mut self) -> &mut Self {
        // Build the variant
        let vis = syn::Visibility::Inherited;

        self.vis = vis;
        self
    }

    ///
    /// Marks the function as `extern`, with an optional `abi` specifier
    ///
    pub fn external(&mut self, abi: Option<&str>) -> &mut Self {
        // Convert the abi string into an identifier
        let name = if let Some(abi) = abi {
            Some(syn::LitStr::new(abi, Span::call_site()))
        } else {
            None
        };

        // Build the new `syn::Abi` item
        let abi = syn::Abi {
            extern_token: Default::default(),
            name,
        };

        self.sig.abi = Some(abi);
        self
    }
}

///
/// A builder wrapper around the `syn` crate's `ItemFn` struct. This builder chains the result of a
/// `FunctionSignatureBuilder` to define the signature of the function that a `FunctionBuilder`
/// will emit.
///
/// This builder separates the construction of a function signature from the body of a function as
/// these are two distinct pieces of information. A function signature can be shared by a Rust
/// fn item or an ffi function and so the API of this crate separates these two concepts to remove
/// code duplication.
///
pub struct FunctionBuilder {
    inner: ItemFn,
}

impl FunctionBuilder {
    ///
    /// Creates a new `FunctionBuilder` with a function signature defined by the given
    /// `FunctionSignatureBuilder`
    ///
    pub fn new(signature: FunctionSignatureBuilder) -> Self {
        let block = Block {
            brace_token: Default::default(),
            stmts: Vec::new(),
        };
        let block = Box::new(block);

        Self {
            inner: ItemFn {
                attrs: signature.attrs,
                vis: signature.vis,
                sig: signature.sig,
                block,
            },
        }
    }

    ///
    /// Appends a statement to the end of the list of statements that define the function's body
    ///
    pub fn statement(&mut self, statement: Stmt) -> &mut Self {
        self.inner.block.stmts.push(statement);
        self
    }

    ///
    /// Consumes the builder to finalize the `ItemFn` struct and then returns the built `ItemFn`
    ///
    pub fn build(self) -> ItemFn {
        self.inner
    }
}
