// This file is licensed under the MIT license, and is a modified version of
// https://github.com/RSSchermer/std140.rs. A copy of that license can be found here: in
// the path licenses/RSSchermer/std140.
//
// This file was modified to support web, and use more hygenic module paths.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Ident};

#[proc_macro_attribute]
pub fn uniform(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty(), "#[uniform] does not take arguments.");
    let input = parse_macro_input!(input as DeriveInput);
    parse_uniform(&input).unwrap_or_else(compile_error).into()
}

fn parse_uniform(input: &DeriveInput) -> Result<TokenStream2, String> {
    let data = match &input.data {
        Data::Struct(data) => data,
        _ => return Err("Cannot represent an enum or union with #[uniform], only a struct.".to_string()),
    };

    if input.attrs.iter().any(|attr| attr.path.is_ident("repr")) {
        return Err("Cannot parse another #[repr] attribute on a struct marked with #[uniform]".to_string());
    }

    let mod_path = quote!(std140);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let asserts = data.fields.iter().map(|field| {
        let ty = &field.ty;
        let span = field.span();

        quote_spanned!(span=> assert_std140_element::<#ty> { marker: core::marker::PhantomData };)
    });

    let suffix = struct_name.to_string().trim_start_matches("r#").to_owned();
    let dummy_const = Ident::new(&format!("_IMPL_STORM_UNIFORM_FOR_{}", suffix), Span::call_site());

    let asserts = quote! {
        struct assert_std140_element<T> where T: #mod_path::Std140Element {
            marker: core::marker::PhantomData<T>
        }

        #(#asserts)*
    };

    let impl_std140_struct = quote! {
        #[automatically_derived]
        unsafe impl #impl_generics #mod_path::Std140Struct for #struct_name #ty_generics #where_clause {}
    };

    let generated = quote! {
        #[repr(C, align(16))]
        #input

        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const #dummy_const: () = {
            #[allow(unknown_lints)]
            #[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
            #[allow(rust_2018_idioms)]

            #asserts

            #impl_std140_struct
        };
    };

    Ok(generated)
}

fn compile_error(message: String) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#message);
    }
}
