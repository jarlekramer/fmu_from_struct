use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FmrsModel)]
pub fn fmrs_model_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    impl_fmrs_model(&input)
}

fn impl_fmrs_model(input: &syn::DeriveInput) -> TokenStream {
    let name = &input.ident;
    let data = &input.data;

    let mut tokens = quote!{};

    tokens.into()
}