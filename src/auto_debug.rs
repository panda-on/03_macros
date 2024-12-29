use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
struct AutoDebugReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), AutoDebugFieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(debug))]
struct AutoDebugFieldReceiver {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn impl_auto_debug_macro(ast: &syn::DeriveInput) -> TokenStream {
    let AutoDebugReceiver {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugReceiver::from_derive_input(ast).unwrap()
    else {
        panic!("Only structs are supported")
    };

    let fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let skip = field.skip;
        if skip {
            quote! {}
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
        impl ::core::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()
            }
        }
    }
    .into()
}
