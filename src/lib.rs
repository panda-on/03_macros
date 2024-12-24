// proc macro crate

use proc_macro::TokenStream;
use quote::quote;

// automatic implement From trait of enum variants for enums
#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("input: \n{:#?}", input);
    let ident = input.ident;
    // println!("ident: {}", ident);

    // generate enum variants for every variant
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be used on enums"),
    };

    // for each variant, get the ident and the fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // HACK: only implement one field case
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident {
                            fn from(value: #ty) -> Self {
                                Self::#var(value)
                            }
                        }
                    }
                }
            }

            syn::Fields::Unit => quote! {},
            syn::Fields::Named(_fields) => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
