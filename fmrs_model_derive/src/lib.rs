use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

mod model_description;
mod field_information;

use field_information::FieldInformation;

#[proc_macro_derive(FmrsModel, attributes(parameter, input, output))]
pub fn fmrs_model_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let name_string = name.to_string();

    let fields = FieldInformation::parse_fields(&input);

    let _write_res = model_description::generate_model_description(&name_string, &fields);

    let get_tokens = impl_get_functions(name, &fields);
    let div_tokens = impl_fmrs_model(&input);

    quote! {
        #get_tokens
        #div_tokens
    }.into()
}

fn impl_get_functions(name: &syn::Ident, fields: &Vec<FieldInformation>) -> TokenStream2 {
    let field_names = fields.iter().map(|field| &field.name);
    
    let tokens = quote! {
        pub fn hello_world() {
            println!("Hello!")
        }
    };

    tokens
}


fn impl_fmrs_model(input: &syn::DeriveInput) -> TokenStream2 {
    let tokens = quote!();

    tokens
}