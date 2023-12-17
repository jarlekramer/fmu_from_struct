use proc_macro::TokenStream;

use quote::quote;
use syn;

mod model_description;
mod field_information;
mod model_management;
mod get_and_set;
mod do_step;

use field_information::FieldInformation;

enum FmiVersion {
    Fmi2,
    Fmi3,
}

#[proc_macro_derive(FmrsModel, attributes(fmi_version, parameter, input, output))]
pub fn fmrs_model_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let fmi_version = parse_fmi_version(&input);

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

fn parse_fmi_version(input: &syn::DeriveInput) -> FmiVersion {
    if input.attrs.len() == 0 {
        FmiVersion::Fmi3
    } else {
        let attributes = &input.attrs;

        for attribute in attributes {
            let meta = &attribute.meta;

            match meta {
                syn::Meta::NameValue(data) => {
                    dbg!(&data.value);
                },
                _ => {}
            }

        }

        FmiVersion::Fmi3
    }
}