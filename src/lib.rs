mod auto_debug;
mod auto_deref;
mod enum_from;
mod enum_from_darling;

use auto_debug::impl_auto_debug_macro;
use auto_deref::impl_auto_deref_macro;
use enum_from::impl_enum_from_macro;
use enum_from_darling::impl_enum_from_macro_darling;
use proc_macro::TokenStream;

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

#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_auto_deref_macro(&ast)
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_auto_debug_macro(&ast)
}
