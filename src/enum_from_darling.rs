use darling::{
    ast::{Data, Fields, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro::TokenStream;
use quote::quote;

#[derive(FromDeriveInput, Debug)]
struct EnumFromDarling {
    ident: syn::Ident,
    data: Data<EnumVariants, ()>,
    generics: syn::Generics,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<EnumVariantFields>,
}

#[derive(Debug, FromField)]
struct EnumVariantFields {
    ty: syn::Type,
}

pub(crate) fn impl_enum_from_macro_darling(ast: &syn::DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        data: Data::Enum(data),
        generics,
    } = EnumFromDarling::from_derive_input(ast).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };

    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            // HACK: only support tuple structure with 1 field
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            Self::#var(v)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    quote! {
        #(#from_impls)*
    }
    .into()
}
