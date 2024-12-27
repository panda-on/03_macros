use enum_from::impl_enum_from_macro;
use enum_from_darling::impl_enum_from_macro_darling;
use proc_macro::TokenStream;

mod enum_from;
mod enum_from_darling;

#[proc_macro_derive(EnumFrom)]
pub fn enum_from(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_enum_from_macro(&ast)
}

#[proc_macro_derive(EnumFromDarling)]
pub fn enum_from_darling(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_enum_from_macro_darling(&ast)
}
