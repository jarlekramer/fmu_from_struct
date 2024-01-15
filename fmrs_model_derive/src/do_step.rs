//! Implements the `do_step` function for the generated FMU struct.


use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use crate::fmi_version::FmiVersion;

pub fn impl_do_step(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi2DoStep(
                instance: *mut ffi::c_void,
                current_communication_point: f64,
                communication_step_size: f64,
                _no_set_fmu_state_prior_to_current_point: bool,
            ) -> fmi2Status
        },
        FmiVersion::Fmi3 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi3DoStep(
                instance: *mut ffi::c_void,
                current_communication_point: f64,
                communication_step_size: f64,
                _no_set_fmu_state_prior_to_current_point: bool,
                _event_handling_needed: *mut bool,
                _terminate_simulation: *mut bool,
                _early_return: *mut bool,
                _last_successful_time: *mut f64,
            ) -> fmi3Status
        },
    };

    let return_value = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2Status::fmi2OK },
        FmiVersion::Fmi3 => quote! { fmi3Status::fmi3OK },
    };

    let tokens = quote! {
        #function_signature {
            unsafe {
                let model: &mut #structure_name = &mut *(instance as *mut #structure_name);

                model.do_step(current_communication_point, communication_step_size);
            }
            
            #return_value
        }
    
    };

    tokens
}