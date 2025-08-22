//! Module for generating the code for "model management" functions. This means the following:
//! - initialization of the model before the parameters are read from the model description
//! - updating the model with the parameters from the model description
//! - freeing the model when the simulation is done

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use crate::fmi_version::FmiVersion;

pub fn get_instance(structure_name: &syn::Ident) -> TokenStream2 {
    quote! {
        let instance: &mut #structure_name = &mut *(instance_ptr as *mut #structure_name);
    }
}

pub fn impl_init_functions(fmi_version: FmiVersion, model_name: &syn::Ident, fmu_info_field_name: Option<syn::Ident>) -> TokenStream2 {
    let instantiate_tokens = impl_instantiate(fmi_version, model_name, fmu_info_field_name.clone());
    let enter_tokens = impl_enter_initialization_mode(fmi_version);
    let exit_tokens = impl_exit_initialization_mode(fmi_version, model_name);
    let reset_tokes = impl_reset(fmi_version, model_name, fmu_info_field_name.clone());

    quote! {
        #instantiate_tokens
        #enter_tokens
        #exit_tokens
        #reset_tokes
    }
}

/// First initialization the model, before the parameters are read from the model description.
fn impl_instantiate(fmi_version: FmiVersion, structure_name: &syn::Ident, fmu_info_field_name: Option<syn::Ident>) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! { 
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi2Instantiate(
                instance_name: *const ffi::c_char,
                _fmu_type: fmi2Type,
                _fmu_guid: *const ffi::c_char,
                resource_path: *const ffi::c_char,
                _functions: fmi2CallbackFunctions,
                _visible: bool,
                _logging_on: bool,
            ) -> *mut ffi::c_void 
        },
        FmiVersion::Fmi3 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi3InstantiateCoSimulation(
                instance_name: *const ffi::c_char,
                _instantiation_token: *const ffi::c_char,
                resource_path: *const ffi::c_char,
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

    let fmu_info_code = if let Some(field_name) = fmu_info_field_name {
        quote! {
            let resource_path_string: String = ffi::CStr::from_ptr(resource_path)
                .to_string_lossy()
                .into_owned();

            let resource_path_string = resource_path_string
                .strip_prefix("file:///")
                .unwrap_or(&resource_path_string)
                .to_string();

            let resource_path_string = resource_path_string.replace("/", "\\");

            let resource_path_buf: PathBuf = PathBuf::from(resource_path_string);

            instance.#field_name = FmuInfo {
                name: instance_name,
                resource_path: resource_path_buf,
            };
        }
    } else {
        quote! {}
    };
    
    quote! {
        #function_signature {
            unsafe {
                let instance_name: String = ffi::CStr::from_ptr(instance_name).to_string_lossy().into_owned();
                
                // The box is needed to avoid the model to be dropped when it goes out of scope.
                let mut instance = Box::new(
                    #structure_name::default()
                );

                #fmu_info_code

                let ptr = Box::into_raw(instance) as *mut _;

                ptr as *mut ffi::c_void
            }
        }
    }
}

fn impl_enter_initialization_mode(fmi_version: FmiVersion) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => {
            quote! { fmi2EnterInitializationMode(instance_ptr: *mut ffi::c_void) -> FmiStatus }
        },
        FmiVersion::Fmi3 => {
            quote! { fmi3EnterInitializationMode(
                instance_ptr: *mut ffi::c_void,
                tolerance_defined: bool,
                tolerance: f64,
                start_time: f64,
                stop_time_defined: bool,
                stop_time: f64,
            ) -> FmiStatus }
        },
    };

    // Returns code that currently does nothing...
    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_signature {
            FmiStatus::Ok
        }
    }
}

fn impl_exit_initialization_mode(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => {
            quote! { fmi2ExitInitializationMode(instance_ptr: *mut ffi::c_void) -> FmiStatus }
        },
        FmiVersion::Fmi3 => {
            quote! { fmi3ExitInitializationMode(instance_ptr: *mut ffi::c_void) -> FmiStatus }
        },
    };

    let instance_tokens = get_instance(structure_name);

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_signature {
            unsafe {
                #instance_tokens;

                instance.exit_initialization_mode();
            }

            FmiStatus::Ok
        }
    }
}


/// Free the model when the simulation is done.
pub fn impl_free_instance(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2FreeInstance },
        FmiVersion::Fmi3 => quote! { fmi3FreeInstance },
    };

    let instance_tokens = get_instance(structure_name);

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(instance_ptr: *mut ffi::c_void) {
            if !instance_ptr.is_null() {
                #instance_tokens;

                let _box = Box::from_raw(instance);
                // _box is dropped here and memory is deallocated
            }
        }
    }
}

/// Reset the model if necessary
pub fn impl_reset(fmi_version: FmiVersion, structure_name: &syn::Ident, fmu_info_field_name: Option<syn::Ident>) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2Reset },
        FmiVersion::Fmi3 => quote! { fmi3Reset },
    };

    let instance_tokens = get_instance(structure_name);

    let fmu_info_clone_tokens = if let Some(field_name) = &fmu_info_field_name {
        quote! {let fmu_info = instance.#field_name.clone() }
    } else {
        quote! {}
    };

    let fmu_info_set_tokens = if let Some(field_name) = &fmu_info_field_name {
        quote! {
            instance.#field_name = fmu_info
        }
    } else {
        quote! {}
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(instance_ptr: *mut ffi::c_void) -> FmiStatus {
            if instance_ptr.is_null() {
                return FmiStatus::Error;
            }

            #instance_tokens;

            #fmu_info_clone_tokens;

            let new_structure = #structure_name::default();

            *instance = new_structure;

            #fmu_info_set_tokens;

            FmiStatus::Ok
        }
    }
}

