use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

pub fn get_superstructure_name(structure_name: &syn::Ident) -> syn::Ident {
    quote::format_ident!("{}Superstructure", structure_name)
}

pub fn impl_superstructure(structure_name: &syn::Ident) -> TokenStream2 {
    let superstructre_name = get_superstructure_name(structure_name);
    
    quote! {
        #[derive(Debug)]
        pub struct #superstructre_name {
            pub instance_name: String,
            pub model: #structure_name,
        }
    }
}