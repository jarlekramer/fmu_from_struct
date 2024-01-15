//! Implements the getters and setters

use proc_macro2::TokenStream as TokenStream2;
use proc_macro2;

use quote::quote;
use syn;

use super::field_information::FieldInformation;
use super::fmi_version::FmiVersion;

fn get_rust_types(fmi_version: FmiVersion) -> Vec<syn::Ident> {
    let mut rust_types: Vec<syn::Ident> = Vec::new();

    rust_types.push(syn::Ident::new("f64",  proc_macro2::Span::call_site()));
    rust_types.push(syn::Ident::new("bool", proc_macro2::Span::call_site()));
    rust_types.push(syn::Ident::new("i32",  proc_macro2::Span::call_site()));

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

fn impl_get_function(
    fmi_version: FmiVersion, 
    name: &syn::Ident, 
    all_fields: &[FieldInformation], 
    field_type: &syn::Ident
) -> TokenStream2 {
    let function_name = quote::format_ident!(
        "{}Get{}", 
        fmi_version.to_function_string(), 
        FieldInformation::get_fmi_type_name(fmi_version, field_type)
    );

    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name(
                instance: *mut ffi::c_void,
                value_references: *const u32,
                n_value_references: usize,
                values: *mut #field_type,
            ) -> fmi2Status
        },
        FmiVersion::Fmi3 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name(
                instance: *mut ffi::c_void,
                value_references: *const u32,
                n_value_references: usize,
                values: *mut #field_type,
                _n_values: usize,
            ) -> fmi3Status
        },
    };

    let filtered_fields = FieldInformation::filter_on_type(all_fields, field_type);

    if filtered_fields.is_empty() {
        match fmi_version {
            FmiVersion::Fmi2 => quote! {
                #function_signature {
                    fmi2Status::fmi2Error
                }
            },
            FmiVersion::Fmi3 => quote! {
                #function_signature {
                    fmi3Status::fmi3Error
                }
            },
        }
    } else {
        let field_names            = filtered_fields.iter().map(|field| &field.name);
        let field_value_references = filtered_fields.iter().map(|field| field.value_reference);

        let return_value = match fmi_version {
            FmiVersion::Fmi2 => quote! {
                fmi2Status::fmi2OK
            },
            FmiVersion::Fmi3 => quote! {
                fmi3Status::fmi3OK
            },
        };

        quote! {
            #function_signature {
                let ptr = instance as *mut #name;
            
                unsafe {
                    let model: &mut #name = &mut *ptr;

                    for i in 0..n_value_references {
                        let input_value_reference = *value_references.offset(i as isize) as usize;

                        match input_value_reference {
                            #(
                                #field_value_references => {
                                    *values.offset(i as isize) = model.#field_names;
                                }
                            )*
                            _ => {panic!("Unknown value reference: {}", input_value_reference)}
                        }
                    }

                }
                
                #return_value
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
    let function_name = quote::format_ident!(
        "{}Set{}", 
        fmi_version.to_function_string(), 
        FieldInformation::get_fmi_type_name(fmi_version, field_type)
    );

    let function_signature = match fmi_version {
        FmiVersion::Fmi2 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name(
                instance: *mut ffi::c_void,
                value_references: *const u32,
                n_value_references: usize,
                values: *mut #field_type,
            ) -> fmi2Status
        },
        FmiVersion::Fmi3 => quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name(
                instance: *mut ffi::c_void,
                value_references: *const u32,
                n_value_references: usize,
                values: *mut #field_type,
                _n_values: usize,
            ) -> fmi3Status
        },
    };

    let filtered_fields = FieldInformation::filter_on_type(all_fields, field_type);

    if filtered_fields.is_empty() {
        match fmi_version {
            FmiVersion::Fmi2 => quote! {
                #function_signature {
                    fmi2Status::fmi2Error
                }
            },
            FmiVersion::Fmi3 => quote! {
                #function_signature {
                    fmi3Status::fmi3Error
                }
            },
        }
    } else {
        let field_names            = filtered_fields.iter().map(|field| &field.name);
        let field_value_references = filtered_fields.iter().map(|field| field.value_reference);

        let return_value = match fmi_version {
            FmiVersion::Fmi2 => quote! {
                fmi2Status::fmi2OK
            },
            FmiVersion::Fmi3 => quote! {
                fmi3Status::fmi3OK
            },
        };
        
        quote! {
            #function_signature {
                let ptr = instance as *mut #name;
            
                unsafe {
                    let model: &mut #name = &mut *ptr;

                    for i in 0..n_value_references {
                        let input_value_reference = *value_references.offset(i as isize) as usize;

                        match input_value_reference {
                            #(
                                #field_value_references => {
                                    model.#field_names = *values.offset(i as isize);
                                }
                            )*
                            _ => {panic!("Unknown value reference: {}", input_value_reference)}
                        }
                    }
                }
                
                #return_value
            }
        }
    }
}