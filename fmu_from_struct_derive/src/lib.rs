use proc_macro::TokenStream;
use quote::quote;
use syn;

mod model_description;
mod field_information;
mod model_management;
mod get_and_set;
mod do_step;
mod fmi_version;
mod state_management;
mod fmu_info;

use field_information::FieldInformation;
use fmi_version::FmiVersion;


#[proc_macro_derive(Fmu, attributes(fmu_from_struct))]
/// The function that implements the derive macro. It starts by passing the input token stream, 
/// with the help from the syn crate. In particular, the goal is to create different data structures 
/// with relevant information about the structure. Then, the parsed data structures are used to
/// generate new functions for the structure that makes it support the FMI-standard.
/// 
/// Most of the functionality in this crate is kept private, as it is only intended to be used in 
/// this function. To see documentation for it run `cargo doc --document-private-items --open` in 
/// the crate folder.
pub fn fmu_from_struct_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    
    // Parse the input
    let name = &input.ident;
    let fmi_version = FmiVersion::parse(&input);
    let fields: Vec<FieldInformation> = FieldInformation::parse(&input);

    let fmu_info_field_name = fmu_info::search_for_fmu_info_field_name(&input);

    // Generate the model description. It is written to file, so that it can be inspected, and
    // embedded in the compiled library, so that it can be packaged together with the library.
    let model_description = model_description::generate_model_description(
        fmi_version,
        &name.to_string(),
        &fields
    );

    let _write_res = model_description::write_model_description(&model_description);

    // Generate the code for the functional mockup interface
    let version_tokens = fmi_version.impl_fmi_version();
    
    let init_tokens = model_management::impl_init_functions(
        fmi_version, 
        name, 
        fmu_info_field_name,
        &fields
    );
    
    let get_tokens     = get_and_set::impl_get_functions(fmi_version, name, &fields);
    let set_tokens     = get_and_set::impl_set_functions(fmi_version, name, &fields);
    let do_step_tokens = do_step::impl_do_step(fmi_version, name);
    let state_tokens   = state_management::impl_state_managment(fmi_version, name);
    let free_tokens    = model_management::impl_free_instance(fmi_version, name);
    let embedded_model_description_tokens = model_description::impl_embedded_model_description(
        &model_description
    );

    quote! {
        #version_tokens
        #init_tokens
        #get_tokens
        #set_tokens
        #do_step_tokens
        #state_tokens
        #free_tokens
        #embedded_model_description_tokens
    }.into()
}
