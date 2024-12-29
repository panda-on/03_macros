use proc_macro::TokenStream;
use quote::quote;

/// automatic implement From trait of enum variants for enums
/// example:
/// ```
/// use enum_from::EnumFrom;
/// #[derive(EnumFrom)]
/// enum pancake{
///     A,
/// };
///
/// struct A;
///
/// fn main() {
///    let a = A;
///    let pancake = pancake::A.from(a);
/// }
/// ```
pub(crate) fn impl_enum_from_macro(ast: &syn::DeriveInput) -> TokenStream {
    // get the ident of the enum
    let ident = &ast.ident;
    // get the generics of the enum
    let generics = &ast.generics;
    // generate enum variants for every variant
    let variants = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => panic!("EnumFrom can only be used on enums"),
    };
    println!("{:?}", &ast);
    // for each variant, get the ident and the fields
    let from_impls = variants.iter().map(|variant| {
        // get the ident of the variant
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
                        impl #generics From<#ty> for #ident #generics {
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
