use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

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


#[proc_macro_derive(Fmu, attributes(fmi_version, parameter, input, output))]
pub fn fmu_from_struct_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    
    // Parse the input
    let name = &input.ident;
    let fmi_version = FmiVersion::parse(&input);
    let fields = FieldInformation::parse(&input);

    let fmu_info_field_name = fmu_info::search_for_fmu_info_field_name(&input);

    // Write the model description to file
    let _write_res = model_description::generate_model_description(fmi_version, &name.to_string(), &fields);

    // Generate the code for the fmi interface
    let version_tokens = impl_fmi_version(fmi_version);
    let init_tokens    = model_management::impl_init_functions(fmi_version, name, fmu_info_field_name);
    let get_tokens     = get_and_set::impl_get_functions(fmi_version, name, &fields);
    let set_tokens     = get_and_set::impl_set_functions(fmi_version, name, &fields);
    let do_step_tokens = do_step::impl_do_step(fmi_version, name);
    let state_tokens   = state_management::impl_state_managment(fmi_version, name);
    let free_tokens    = model_management::impl_free_instance(fmi_version, name);

    quote! {
        #version_tokens
        #init_tokens
        #get_tokens
        #set_tokens
        #do_step_tokens
        #state_tokens
        #free_tokens
    }.into()
}


fn impl_fmi_version(fmi_version: FmiVersion) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2GetVersion },
        FmiVersion::Fmi3 => quote! { fmi3GetVersion },
    };

    let fmi_version_token = match fmi_version {
        FmiVersion::Fmi2 => quote! { "2.0\0" },
        FmiVersion::Fmi3 => quote! { "3.0\0" },
    };

    let version_tokens =  quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_name() -> *const ffi::c_char {
            #fmi_version_token.as_ptr() as *const ffi::c_char
        }
    };

    let types_platform_tokens = if fmi_version == FmiVersion::Fmi2 {
        quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi2GetTypesPlatform() -> *const ffi::c_char {
                "default\0".as_ptr() as *const ffi::c_char
            }
        }
    } else {
        quote! {}
    };

   quote! {
        #version_tokens
        #types_platform_tokens
    }
}
