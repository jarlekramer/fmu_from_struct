//! Module for generating the code for "model management" functions. This means the following:
//! - initialization of the model before the parameters are read from the model description
//! - udating the model with the parameters from the model description
//! - freeing the model when the simulation is done

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use crate::fmi_version::FmiVersion;

pub fn impl_init_functions(fmi_version: FmiVersion, model_name: &syn::Ident) -> TokenStream2 {
    let instanciate_tokens = impl_instantiate(fmi_version, model_name);
    let enter_tokens = impl_enter_initialization_mode(fmi_version);
    let exit_tokens = impl_exit_initialization_mode(fmi_version, model_name);

    quote! {
        #instanciate_tokens
        #enter_tokens
        #exit_tokens
    }
}

/// First initialization the model, before the parameters are read from the model description.
fn impl_instantiate(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! { 
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi2Instantiate(
                _instance_name: *const ffi::c_char,
                _fmu_type: fmi2Type,
                _fmu_guid: *const ffi::c_char,
                _fmu_resource_location: *const ffi::c_char,
                _functions: fmi2CallbackFunctions,
                _visible: bool,
                _logging_on: bool,
            ) -> *mut ffi::c_void 
        },
        FmiVersion::Fmi3 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi3InstantiateCoSimulation(
                _instance_name: *const ffi::c_char,
                _instantiation_token: *const ffi::c_char,
                _resource_path: *const ffi::c_char,
                _visible: bool,
                _logging_on: bool,
                _event_mode_used: bool,
                _early_return_allowed: bool,
                _required_intermediate_variables: *const u32,
                _n_required_intermediate_variables: usize,
                _instance_environment: *mut ffi::c_void,
                _log_message: fmi3LogMessageCallback,
                _intermediate_update: fmi3IntermediateUpdateCallback,
            ) -> *mut ffi::c_void 
        },
    };
    
    quote! {
        #function_signature {
            // The box is needed to avoid the model to be dropped when it goes out of scope.
            let mut model = Box::new(#structure_name::default());

            let ptr = Box::into_raw(model) as *mut _;

            ptr as *mut ffi::c_void
        }
    }
}

fn impl_enter_initialization_mode(fmi_version: FmiVersion) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => {
            quote! { fmi2EnterInitializationMode(instance: *mut ffi::c_void) -> fmi2Status }
        },
        FmiVersion::Fmi3 => {
            quote! { fmi3EnterInitializationMode(
                instance: *mut ffi::c_void,
                tolerance_defined: bool,
                tolerance: f64,
                start_time: f64,
                stop_time_defined: bool,
                stop_time: f64,
            ) -> fmi3Status }
        },
    };


    let return_value = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2Status::fmi2OK },
        FmiVersion::Fmi3 => quote! { fmi3Status::fmi3OK },
    };

    // Returns code that currently does nothing...
    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_signature {
            #return_value
        }
    }
}

fn impl_exit_initialization_mode(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => {
            quote! { fmi2ExitInitializationMode(instance: *mut ffi::c_void) -> fmi2Status }
        },
        FmiVersion::Fmi3 => {
            quote! { fmi3ExitInitializationMode(instance: *mut ffi::c_void) -> fmi3Status }
        },
    };

    let return_value = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2Status::fmi2OK },
        FmiVersion::Fmi3 => quote! { fmi3Status::fmi3OK },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_signature {
            unsafe {
                let model: &mut #structure_name = &mut *(instance as *mut #structure_name);

                model.exit_initialization_mode();
            }

            #return_value
        }
    }
}


/// Free the model when the simulation is done.
pub fn impl_free_instance(fmi_version: FmiVersion, model_name: &syn::Ident) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2FreeInstance },
        FmiVersion::Fmi3 => quote! { fmi3FreeInstance },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(instance: *mut ffi::c_void) {
            if !instance.is_null() {
                let instance = instance as *mut #model_name;

                let _box = Box::from_raw(instance);
                // _box is dropped here and memory is deallocated
            }
        }
    }
}

