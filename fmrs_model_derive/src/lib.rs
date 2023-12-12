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
    
    let data: &syn::Data = &input.data;

    let tokens = match *data {
        syn::Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    println!("Found named fields");

                    let field_names = fields.named.iter().map(|f| {&f.ident});

                    quote!{
                        impl #name {
                            pub fn new() -> Self {
                                Self {
                                    #(#field_names: 0.0),*
                                }
                            }
                        }
                    }
                },
                syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!(),
            }
        },
        syn::Data::Enum(_) | syn::Data::Union(_) =>  unimplemented!(),
    };

    tokens.into()
}