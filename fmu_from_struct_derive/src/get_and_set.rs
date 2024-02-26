//! Implements the getters and setters

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2;

use quote::quote;
use syn;

use super::field_information::FieldInformation;
use super::fmi_version::FmiVersion;
use super::model_management::get_instance;


/// Generates the getter functions for all variable types
pub fn impl_get_functions(fmi_version: FmiVersion, name: &syn::Ident, fields: &[FieldInformation]) -> TokenStream2 {
    let rust_types = get_rust_types(fmi_version);

    let mut functions: Vec<TokenStream2> = Vec::new();

    for t in rust_types {
        functions.push(impl_get_function(fmi_version, name, fields, &t));
    }

    quote! {
        #(#functions)*
    }
}

/// Generates the getter functions for all variable types
pub fn impl_set_functions(fmi_version: FmiVersion, name: &syn::Ident, fields: &[FieldInformation]) -> TokenStream2 {
    let rust_types = get_rust_types(fmi_version);

    let mut functions: Vec<TokenStream2> = Vec::new();

    for t in rust_types {
        functions.push(impl_set_function(fmi_version, name, fields, &t));
    }

    quote! {
        #(#functions)*
    }
}

/// Defines the rust types that should get getters and setters, based on the FMI version
fn get_rust_types(fmi_version: FmiVersion) -> Vec<syn::Ident> {
    let mut rust_types: Vec<syn::Ident> = Vec::new();

    rust_types.push(syn::Ident::new("f64",  proc_macro2::Span::call_site()));
    rust_types.push(syn::Ident::new("bool", proc_macro2::Span::call_site()));
    rust_types.push(syn::Ident::new("i32",  proc_macro2::Span::call_site()));
    rust_types.push(syn::Ident::new("String", proc_macro2::Span::call_site()));

    if let FmiVersion::Fmi3 = fmi_version {
        rust_types.push(syn::Ident::new("f32", proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("i8",  proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("i16", proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("i64", proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("u8",  proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("u16", proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("u32", proc_macro2::Span::call_site()));
        rust_types.push(syn::Ident::new("u64", proc_macro2::Span::call_site()));
    }

    rust_types
}


// Enum to distinguish between getter and setter functions in the function signature
enum Access {
    Get,
    Set,
}

/// Generates function signature for getter and setter functions based on fmi version and field type
fn get_function_signature(fmi_version: FmiVersion, field_type: &syn::Ident, access: Access) -> TokenStream2 {
    let access_type = match access {
        Access::Get => "Get",
        Access::Set => "Set",
    };

    let function_name = quote::format_ident!(
        "{}{}{}", 
        fmi_version.to_function_string(), 
        access_type,
        FieldInformation::get_fmi_type_name(fmi_version, field_type)
    );

    let c_field_tokens = if field_type.to_string() == "String" {
        match access {
            Access::Get => quote! { *mut *const ffi::c_char },
            Access::Set => quote! { *const *const ffi::c_char },
        }
    } else {
        match access {
            Access::Get => quote! { *mut #field_type },
            Access::Set => quote! { *const #field_type },
        }
    };

    match fmi_version {
        FmiVersion::Fmi2 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name(
                instance_ptr: *mut ffi::c_void,
                value_references: *const u32,
                n_value_references: usize,
                values: #c_field_tokens,
            ) -> FmiStatus
        },
        FmiVersion::Fmi3 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name(
                instance_ptr: *mut ffi::c_void,
                value_references: *const u32,
                n_value_references: usize,
                values: #c_field_tokens,
                _n_values: usize,
            ) -> FmiStatus
        },
    }
}

fn impl_get_function(
    fmi_version: FmiVersion, 
    name: &syn::Ident, 
    all_fields: &[FieldInformation], 
    field_type: &syn::Ident
) -> TokenStream2 {
    let function_signature = get_function_signature(fmi_version, field_type, Access::Get);

    let filtered_fields = FieldInformation::filter_on_type(all_fields, field_type);

    let instance_tokens = get_instance(name);

    if filtered_fields.is_empty() {
        quote! {
            #function_signature {
                FmiStatus::Error
            }
        }
    } else {
        let field_names            = filtered_fields.iter().map(|field| &field.name);
        let field_value_references = filtered_fields.iter().map(|field| field.value_reference);

        if field_type.to_string() == "String" {
            quote! {
                #function_signature {
                    unsafe {
                        #instance_tokens;
    
                        for i in 0..n_value_references {
                            let input_value_reference = *value_references.offset(i as isize) as usize;
    
                            match input_value_reference {
                                #(
                                    #field_value_references => {
                                        let rust_string = instance.model.#field_names.clone();

                                        let c_string = ffi::CString::new(rust_string).unwrap();

                                        *values.offset(i as isize) = c_string.into_raw();
                                    }
                                )*
                                _ => {panic!("Unknown value reference: {}", input_value_reference)} // Consider changing this to return an error
                            }
                        }
                    }
                    
                    FmiStatus::Ok
                }   
            }
        } else {
            quote! {
                #function_signature {
                    unsafe {
                        #instance_tokens;
    
                        for i in 0..n_value_references {
                            let input_value_reference = *value_references.offset(i as isize) as usize;
    
                            match input_value_reference {
                                #(
                                    #field_value_references => {
                                        *values.offset(i as isize) = instance.model.#field_names;
                                    }
                                )*
                                _ => {panic!("Unknown value reference: {}", input_value_reference)} // Consider changing this to return an error
                            }
                        }
                    }
                    
                    FmiStatus::Ok
                }
            }
        }
    }
}

fn impl_set_function(
    fmi_version: FmiVersion, 
    name: &syn::Ident, 
    all_fields: &[FieldInformation], 
    field_type: &syn::Ident
) -> TokenStream2 {   
    let function_signature = get_function_signature(fmi_version, field_type, Access::Set);

    let filtered_fields = FieldInformation::filter_on_type(all_fields, field_type);

    let instance_tokens = get_instance(name);

    if filtered_fields.is_empty() {
        quote! {
            #function_signature {
                FmiStatus::Error
            }
        }
    } else {
        let field_names            = filtered_fields.iter().map(|field| &field.name);
        let field_value_references = filtered_fields.iter().map(|field| field.value_reference);

        let set_value_at_index = if field_type.to_string() == "String" {
            quote! {
                let c_str: &ffi::CStr = ffi::CStr::from_ptr(*values.offset(i as isize));

                let rust_string: String = c_str.to_string_lossy().into_owned();
            
                let value = rust_string
            }
        } else {
            quote! {
                let value = *values.offset(i as isize)
            }
        };

        quote! {
            #function_signature {
                unsafe {
                    #instance_tokens;

                    for i in 0..n_value_references {
                        let input_value_reference = *value_references.offset(i as isize) as usize;

                        match input_value_reference {
                            #(
                                #field_value_references => {
                                    #set_value_at_index;

                                    instance.model.#field_names = value;
                                }
                            )*
                            _ => {panic!("Unknown value reference: {}", input_value_reference)} // Consider changing this to return an error
                        }
                    }
                }
                
                FmiStatus::Ok
            }
        }
    }
}