//! Functionality to manage the "FMU state" (currently not implemented properly, all functions only
//! returns errors...)
//! 
//! The FMU state is meant to allow for restart of a simulation from a given point. Examples can be 
//! that a "do_step" call fails, and the FMU must be rolled back to the state before the call, in
//! order to try again / try something different.
//! 
//! A state is not explicitly defined in the standard, but any data necessary to achieve the above 
//! goal must somehow be copied from the model instance. The details of how this is done is decided 
//! in the set and get state functions.
//! 
//! In addition, the states should be serializable as bytes.
//! 
//! WARNING: Not sure how to deal with memory in between setting and getting the state. Should the
//! previous state be freed before a new state is stored? Or should the state be copied? Is it
//! necessary to deal with varying sizes of the state?
//! 
//! These functions should not be called unless the feature is enabled in the modelExchange file 
//! (which it is not for this crate).

use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use super::fmi_version::FmiVersion;

pub fn impl_state_managment(fmi_version: FmiVersion, structure_name: &syn::Ident) -> TokenStream2 {
    let get_state_tokens  = impl_get_state(fmi_version, structure_name);
    let set_state_tokens  = impl_set_state(fmi_version, structure_name);
    let free_state_tokens = impl_free_state(fmi_version, structure_name);
    let serialize_state_tokens = impl_serialize_state(fmi_version, structure_name);
    let deserialize_state_tokens = impl_deserialize_state(fmi_version, structure_name);
    let serialized_state_size_tokens = impl_serialized_state_size(fmi_version, structure_name);

    quote! {
        #get_state_tokens
        #set_state_tokens
        #serialize_state_tokens
        #deserialize_state_tokens
        #serialized_state_size_tokens
        #free_state_tokens
    }
}

fn impl_get_state(fmi_version: FmiVersion, _structure_name: &syn::Ident) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2GetFMUstate },
        FmiVersion::Fmi3 => quote! { fmi3GetFMUState },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(
            instance_ptr: *const ffi::c_void,
            state_ptr: *mut *mut ffi::c_void,
        ) -> FmiStatus {
            FmiStatus::Error
        }
    }
}

fn impl_set_state(fmi_version: FmiVersion, _structure_name: &syn::Ident) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2SetFMUstate },
        FmiVersion::Fmi3 => quote! { fmi3SetFMUState },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_name(
            instance_ptr: *mut ffi::c_void,
            state_ptr: *mut ffi::c_void,
        ) -> FmiStatus {
            FmiStatus::Error
        }
    }
}

fn impl_free_state(fmi_version: FmiVersion, _structure_name: &syn::Ident) -> TokenStream2 {    
    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! { 
            fmi2FreeFMUstate(instance_ptr: *const ffi::c_void, state_ptr: *mut ffi::c_void) -> FmiStatus 
        },
        FmiVersion::Fmi3 => quote! {
            fmi3FreeFMUState(instance_ptr: *const ffi::c_void, state_ptr: *mut *mut ffi::c_void) -> FmiStatus
        },
    };
    
    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #function_signature {
            FmiStatus::Error
        }
    }
}

fn impl_serialize_state(fmi_version: FmiVersion, _structure_name: &syn::Ident) -> TokenStream2 {
    let serialize_function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2SerializeFMUstate },
        FmiVersion::Fmi3 => quote! { fmi3SerializeFMUState },
    };

    let serialized_state_type = match fmi_version {
        FmiVersion::Fmi2 => quote! { *mut ffi::c_char },
        FmiVersion::Fmi3 => quote! { *mut u8 },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #serialize_function_name(
            instance_ptr: *mut ffi::c_void,
            state_ptr: *mut ffi::c_void,
            serialized_state: #serialized_state_type,
            size: usize,
        ) -> FmiStatus {
            FmiStatus::Error
        }
    }
}

fn impl_deserialize_state(fmi_version: FmiVersion, _structure_name: &syn::Ident) -> TokenStream2 {
    let serialized_state_type = match fmi_version {
        FmiVersion::Fmi2 => quote! { *const ffi::c_char },
        FmiVersion::Fmi3 => quote! { *const u8 },
    };

    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => {
            quote! {
                fmi2DeSerializeFMUstate(
                    instance_ptr: *mut ffi::c_void,
                    state_ptr: *mut ffi::c_void,
                    serialized_state: #serialized_state_type,
                    size: usize,
                )
            }
        },
        FmiVersion::Fmi3 => {
            quote! {
                fmi3DeserializeFMUState(
                    instance_ptr: *mut ffi::c_void,
                    serialized_state: #serialized_state_type,
                    size: usize,
                    state_ptr: *mut *mut ffi::c_void,
                )
            }
        }
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_signature -> FmiStatus {
            FmiStatus::Error
        }
    }
}

fn impl_serialized_state_size(fmi_version: FmiVersion, _structure_name: &syn::Ident) -> TokenStream2 {
    let function_name = match fmi_version {
        FmiVersion::Fmi2 => quote! { fmi2SerializedFMUstateSize },
        FmiVersion::Fmi3 => quote! { fmi3SerializedFMUStateSize },
    };

    quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn #function_name(
            instance_ptr: *mut ffi::c_void,
            state_ptr: *mut ffi::c_void,
            size: *mut usize,
        ) -> FmiStatus {
            FmiStatus::Error
        }
    }
}