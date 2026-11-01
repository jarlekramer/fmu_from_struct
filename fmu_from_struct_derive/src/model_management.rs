//! Module for generating the code for "model management" functions. This means the following:
//! - initialization of the model before the parameters are read from the model description
//! - updating the model with the parameters from the model description
//! - freeing the model when the simulation is done

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use crate::fmi_version::FmiVersion;
use crate::field_information::FieldInformation;

/// Generates code that gets the FMU instance wrapper, `fmu_instance`, and the user struct, 
/// `instance`, from the instance pointer.
pub fn get_instance(structure_name: &syn::Ident) -> TokenStream2 {
    quote! {
        let fmu_instance: &mut FmuInstance<#structure_name> = 
            &mut *(instance_ptr as *mut FmuInstance<#structure_name>);
        let instance: &mut #structure_name = &mut fmu_instance.model
    }
}

/// Wraps the code in `std::panic::catch_unwind`, so that panics in user code are not propagated
/// through the C interface, which would abort the program. A panic is converted to the
/// `panic_value`.
pub fn catch_panic(code: TokenStream2, panic_value: TokenStream2) -> TokenStream2 {
    quote! {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { #code })) {
            Ok(value) => value,
            Err(_) => #panic_value,
        }
    }
}

pub fn impl_init_functions(
    fmi_version: FmiVersion, 
    model_name: &syn::Ident, 
    fmu_info_field_name: Option<syn::Ident>,
    fields: &[FieldInformation]
) -> TokenStream2 {
    let instantiate_tokens = impl_instantiate(
        fmi_version, 
        model_name, 
        fmu_info_field_name.clone(),
        fields
    );
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
fn impl_instantiate(
    fmi_version: FmiVersion, 
    structure_name: &syn::Ident, 
    fmu_info_field_name: Option<syn::Ident>,
    fields: &[FieldInformation]
) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! { 
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn fmi2Instantiate(
                instance_name: *const ffi::c_char,
                _fmu_type: fmi2Type,
                _fmu_guid: *const ffi::c_char,
                resource_path: *const ffi::c_char,
                _functions: *const fmi2CallbackFunctions,
                _visible: i32,
                _logging_on: i32,
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
                _log_message: Option<fmi3LogMessageCallback>,
                _intermediate_update: Option<fmi3IntermediateUpdateCallback>,
            ) -> *mut ffi::c_void 
        },
    };

    let fmi_major_version: u32 = match fmi_version {
        FmiVersion::Fmi2 => 2,
        FmiVersion::Fmi3 => 3,
    };

    let fmu_info_code = if let Some(field_name) = fmu_info_field_name {
        quote! {
            instance.#field_name = FmuInfo {
                name: instance_name,
                resource_path: resource_path_from_c_str(resource_path, #fmi_major_version),
            };
        }
    } else {
        quote! {}
    };
    
    // Find the fields with custom start values
    let mut fields_with_start_value: Vec<syn::Ident> = vec![];
    let mut start_value_tokens: Vec<TokenStream2> = vec![];
    for field in fields.iter() {
        if let Some(tokens) = field.start_value_tokens() {
            fields_with_start_value.push(field.name.clone());
            start_value_tokens.push(tokens);
        }
    }

    let instantiate_code = catch_panic(
        quote! {
            // The box is needed to avoid the model to be dropped when it goes out of scope.
            let mut fmu_instance = Box::new(
                FmuInstance::new(#structure_name::default())
            );

            let instance = &mut fmu_instance.model;
            
            // Add custom start values here
            #(
                instance.#fields_with_start_value = #start_value_tokens;
            )*
            
            // Add the FMU info field if present
            #fmu_info_code

            Box::into_raw(fmu_instance) as *mut ffi::c_void
        },
        quote! { ptr::null_mut() }
    );
    
    quote! {
        #function_signature {
            unsafe {
                let instance_name: String = if instance_name.is_null() {
                    String::new()
                } else {
                    ffi::CStr::from_ptr(instance_name).to_string_lossy().into_owned()
                };
                
                #instantiate_code
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

fn impl_exit_initialization_mode(
    fmi_version: FmiVersion, 
    structure_name: &syn::Ident
) -> TokenStream2 {
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => {
            quote! { fmi2ExitInitializationMode(instance_ptr: *mut ffi::c_void) -> FmiStatus }
        },
        FmiVersion::Fmi3 => {
            quote! { fmi3ExitInitializationMode(instance_ptr: *mut ffi::c_void) -> FmiStatus }
        },
    };

    let instance_tokens = get_instance(structure_name);

    let exit_code = catch_panic(
        quote! {
            instance.exit_initialization_mode();

            FmiStatus::Ok
        },
        quote! { FmiStatus::Error }
    );

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_signature {
            unsafe {
                #instance_tokens;

                #exit_code
            }
        }
    }
}


/// Free the model when the simulation is done.
pub fn impl_free_instance(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2FreeInstance },
        FmiVersion::Fmi3 => quote! { fmi3FreeInstance },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(instance_ptr: *mut ffi::c_void) {
            if !instance_ptr.is_null() {
                let _box = Box::from_raw(instance_ptr as *mut FmuInstance<#structure_name>);
                // _box is dropped here and memory is deallocated
            }
        }
    }
}

/// Reset the model if necessary
pub fn impl_reset(
    fmi_version: FmiVersion, 
    structure_name: &syn::Ident, 
    fmu_info_field_name: Option<syn::Ident>
) -> TokenStream2 {
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

    let reset_code = catch_panic(
        quote! {
            #fmu_info_clone_tokens;

            let new_structure = #structure_name::default();

            *instance = new_structure;

            #fmu_info_set_tokens;

            FmiStatus::Ok
        },
        quote! { FmiStatus::Error }
    );

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(instance_ptr: *mut ffi::c_void) -> FmiStatus {
            if instance_ptr.is_null() {
                return FmiStatus::Error;
            }

            #instance_tokens;

            #reset_code
        }
    }
}

