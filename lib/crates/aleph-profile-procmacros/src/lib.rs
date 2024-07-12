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

// This crate is a fork of aclysma/profiling-procmacros
//
// MIT License
//
// Copyright (c) 2020 Philip Degarmo and other contributors
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

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, ImplItem, ItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = parse_macro_input!(item as ItemFn);
    let instrumented_function_name = function.sig.ident.to_string();

    let body = &function.block;
    let new_body: syn::Block = impl_block(body, &instrumented_function_name);

    function.block = Box::new(new_body);

    (quote! {
        #function
    })
    .into()
}

#[proc_macro_attribute]
pub fn skip(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn all_functions(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut content = parse_macro_input!(item as ItemImpl);
    let struct_name = content.self_ty.to_token_stream().to_string();

    'func_loop: for block in &mut content.items {
        // Currently, we only care about the function impl part.
        // In the future, expand the code to following if we are interested in other parts
        //
        // match block {
        //     ImplItem::Fn(ref mut func) => {
        //         for func_attr in &func.attrs {
        //             if let syn::Meta::Path(ref func_attr_info) = func_attr.meta {
        //                 let attr_seg = func_attr_info.segments.last().unwrap();
        //                 if attr_seg.ident.to_string() == "skip".to_string() {
        //                     continue 'func_loop;
        //                 }
        //             }
        //         }
        //         let prev_block = &func.block;
        //         let func_name = func.sig.ident.to_string();
        //         func.block = impl_block(prev_block, &func_name);
        //     }
        //     ImplItem::Macro(_) => { // some code... },
        //     ImplItem::Type(_) => { // some code... },
        //     _ => {}
        // }
        let ImplItem::Fn(ref mut func) = block else {
            continue;
        };

        for func_attr in &func.attrs {
            let func_attr_info = func_attr.path();
            if func_attr_info.segments.is_empty() {
                continue;
            }
            if func_attr_info.segments.first().unwrap().ident != "aleph_profile" {
                continue;
            }
            if func_attr_info.segments.last().unwrap().ident == "skip" {
                continue 'func_loop;
            }
        }
        let prev_block = &func.block;
        let calling_info = format!("{}: {}", struct_name, func.sig.ident);
        func.block = impl_block(prev_block, &calling_info);
    }

    (quote!(
        #content
    ))
    .into()
}

#[cfg(not(any(feature = "profile-with-superluminal", feature = "profile-with-tracy")))]
fn impl_block(body: &syn::Block, _instrumented_function_name: &str) -> syn::Block {
    parse_quote! {
        {
            #body
        }
    }
}

#[cfg(feature = "profile-with-superluminal")]
fn impl_block(body: &syn::Block, instrumented_function_name: &str) -> syn::Block {
    parse_quote! {
        {
            let _superluminal_guard = aleph_profile::detail::Guard::new(#instrumented_function_name);

            #body
        }
    }
}

#[cfg(feature = "profile-with-tracy")]
fn impl_block(body: &syn::Block, instrumented_function_name: &str) -> syn::Block {
    parse_quote! {
        {
            // Note: callstack_depth is 0 since this has significant overhead
            let _tracy_span = aleph_profile::tracy_client::span!(#instrumented_function_name, 0);

            #body
        }
    }
}
