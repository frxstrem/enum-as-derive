extern crate proc_macro;

use heck::SnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Fields, FieldsUnnamed,
    Ident,
};

#[proc_macro_derive(EnumAs)]
pub fn enum_as_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_as(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn enum_as(input: DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let vis = &input.vis;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        Data::Struct(DataStruct { struct_token, .. }) => {
            return Err(syn::Error::new_spanned(
                struct_token,
                "Can only use EnumAs derive macro on enum items",
            ))
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(syn::Error::new_spanned(
                union_token,
                "Can only use EnumAs derive macro on enum items",
            ))
        }
    };

    let mut methods = Vec::new();

    for variant in variants {
        let field = match &variant.fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if unnamed.len() != 1 {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "Can only use EnumAs derive macro on enum items with only newtype variants",
                    ));
                }

                &unnamed[0]
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "Can only use EnumAs derive macro on enum items with only newtype variants",
                ))
            }
        };

        let variant_name = &variant.ident;
        let variant_type = &field.ty;
        let method_name = Ident::new(
            &format!("as_{}", variant_name.to_string().to_snake_case()),
            variant_name.span(),
        );
        let method_name_mut = Ident::new(
            &format!("as_{}_mut", variant_name.to_string().to_snake_case()),
            variant_name.span(),
        );
        let method_name_into = Ident::new(
            &format!("into_{}", variant_name.to_string().to_snake_case()),
            variant_name.span(),
        );

        methods.push(quote! {
            #vis fn #method_name(&self) -> Option<&#variant_type> {
                if let #ident::#variant_name(value) = self {
                    Some(value)
                } else {
                    None
                }
            }

            #vis fn #method_name_mut(&mut self) -> Option<&mut #variant_type> {
                if let #ident::#variant_name(value) = self {
                    Some(value)
                } else {
                    None
                }
            }

            #vis fn #method_name_into(self) -> Option<#variant_type> {
                if let #ident::#variant_name(value) = self {
                    Some(value)
                } else {
                    None
                }
            }
        });
    }

    Ok(quote!{
        impl #ident {
            #( #methods )*
        }
    })
}
