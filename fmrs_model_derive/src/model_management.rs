use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use crate::fmi_version::FmiVersion;

pub fn impl_model_managment(fmi_version: FmiVersion, model_name: &syn::Ident) -> TokenStream2 {
    
    let instantiate_function_signature = match fmi_version {
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
    
    let instanciate_tokens = quote! {
        #instantiate_function_signature {
            // The box is needed to avoid the model to be dropped when it goes out of scope.
            let mut model = Box::new(#model_name::default());

            let ptr = Box::into_raw(model) as *mut _;

            ptr as *mut ffi::c_void
        }
    };

    let free_function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2FreeInstance },
        FmiVersion::Fmi3 => quote! { fmi3FreeInstance },
    };

    let free_tokens = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #free_function_name(instance: *mut ffi::c_void) {
            if !instance.is_null() {
                let instance = instance as *mut #model_name;

                let _box = Box::from_raw(instance);
                // _box is dropped here and memory is deallocated
            }
        }
    };

    quote! {
        #instanciate_tokens
        #free_tokens
    }
}

