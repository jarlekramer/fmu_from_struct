use proc_macro::TokenStream;

use quote::quote;
use syn;

mod model_description;
mod field_information;
mod model_management;
mod get_and_set;
mod do_step;
mod fmi_version;

use field_information::FieldInformation;
use fmi_version::FmiVersion;


#[proc_macro_derive(FmrsModel, attributes(fmi_version, parameter, input, output))]
pub fn fmrs_model_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    
    // Parse the input
    let name = &input.ident;
    let fmi_version = FmiVersion::parse(&input);
    let fields = FieldInformation::parse(&input);

    // Write the model description to file
    let _write_res = model_description::generate_model_description(fmi_version, &name.to_string(), &fields);

    // Generate the code for the fmi interface
    let init_tokens    = model_management::impl_init_functions(fmi_version, name);
    let get_tokens     = get_and_set::impl_get_functions(fmi_version, name, &fields);
    let set_tokens     = get_and_set::impl_set_functions(fmi_version, name, &fields);
    let do_step_tokens = do_step::impl_do_step(fmi_version, name);
    let free_tokens    = model_management::impl_free_instance(fmi_version, name);

    quote! {
        #init_tokens
        #get_tokens
        #set_tokens
        #do_step_tokens
        #free_tokens
    }.into()
}

