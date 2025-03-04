// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(AsVariant)]
pub fn as_variant_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enum_ident = &ast.ident;

    let variants = match ast.data {
        Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("AsVariant can only be applied to enumerations"),
    };

    let as_variant_methods = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let method_ident = format_ident!("as_{}", variant_ident.to_string());
        let variant_type = match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let ty = &fields.unnamed.first().unwrap().ty;
                quote! { #ty }
            }
            _ => panic!("AsVariant can only be applied to unnamed variants"),
        };

        quote! {
            impl #enum_ident {
                pub fn #method_ident(self) -> Option<#variant_type> {
                    match self {
                        #enum_ident::#variant_ident(v) => Some(v),
                        _ => None,
                    }
                }
            }
        }
    });

    let expanded = quote! {
        #(#as_variant_methods)*
    };

    expanded.into()
}
