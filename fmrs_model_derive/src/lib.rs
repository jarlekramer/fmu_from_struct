use proc_macro::TokenStream;

use quote::quote;
use syn;

mod model_description;
mod field_information;
mod model_management;
mod get_and_set;
mod do_step;

use field_information::FieldInformation;

#[proc_macro_derive(FmrsModel, attributes(fmi_version, parameter, input, output))]
pub fn fmrs_model_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let name_string = name.to_string();

    let fields = FieldInformation::parse_fields(&input);

    let _write_res = model_description::generate_model_description(&name_string, &fields);

    let managment_tokens = model_management::impl_model_managment(name);

    let get_tokens = get_and_set::impl_get_functions(name, &fields);
    let set_tokens = get_and_set::impl_set_functions(name, &fields);

    let do_step_tokens = do_step::impl_do_step(name);

    quote! {
        #managment_tokens
        #get_tokens
        #set_tokens
        #do_step_tokens
    }.into()
}

pub fn parse_fmi_version(input: &syn::DeriveInput) -> Option<usize> {
    for attr in &input.attrs {
        if let Ok(syn::Meta::NameValue(meta_name_value)) = attr.parse_meta() {
            if meta_name_value.path.is_ident("fmi_version") {
                if let syn::Lit::Str(lit_str) = meta_name_value.lit {
                    if let Ok(version) = lit_str.value().parse::<usize>() {
                        return Some(version);
                    }
                }
            }
        }
    }

    None
}