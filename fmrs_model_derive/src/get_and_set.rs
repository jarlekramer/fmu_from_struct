use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use super::field_information::FieldInformation;

pub fn impl_get_functions(name: &syn::Ident, fields: &Vec<FieldInformation>) -> TokenStream2 {
    let field_names      = fields.iter().map(|field| &field.name);
    let field_value_references = fields.iter().map(|field| field.value_reference);
    
    let tokens = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3GetFloat64(
            instance: *mut ffi::c_void,
            value_references: *const u32,
            n_value_references: usize,
            values: *mut f64,
            _n_values: usize,
        ) -> fmi3Status {
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
                        _ => {println!("Unknown value reference: {}", input_value_reference)}
                    }
                }

            }
            
            fmi3Status::fmi3OK
        }
    };

    tokens
}

pub fn impl_set_functions(name: &syn::Ident, fields: &Vec<FieldInformation>) -> TokenStream2 {
    let field_names      = fields.iter().map(|field| &field.name);
    let field_value_references = fields.iter().map(|field| field.value_reference);
    
    let tokens = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3SetFloat64(
            instance: *mut ffi::c_void,
            value_references: *const u32,
            n_value_references: usize,
            values: *const f64,
            _n_values: usize,
        ) -> fmi3Status {
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
                        _ => {println!("Unknown value reference: {}", input_value_reference)}
                    }
                }
            }
            
            fmi3Status::fmi3OK
        }
    };

    tokens
}