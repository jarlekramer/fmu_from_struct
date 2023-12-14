use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

use super::field_information::FieldInformation;

pub fn impl_get_functions(name: &syn::Ident, fields: &Vec<FieldInformation>) -> TokenStream2 {
    let field_names      = fields.iter().map(|field| &field.name);
    let value_references = fields.iter().map(|field| field.value_reference);
    
    let tokens = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3GetFloat64(
            instance: fmi3Instance,
            valueReferences: *const fmi3ValueReference,
            _nValueReferences: usize,
            values: *mut f64,
            nValues: usize,
        ) -> fmi3Status {
            let ptr = instance as *mut #name;
        
            unsafe {
                let model: &mut #name = &mut *ptr;

                for i in 0..nValues {
                    let input_value_reference = *valueReferences.offset(i as isize) as usize;

                    match input_value_reference {
                        #(
                            #value_references => {
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
    let value_references = fields.iter().map(|field| field.value_reference);
    
    let tokens = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn fmi3SetFloat64(
            instance: fmi3Instance,
            valueReferences: *const fmi3ValueReference,
            _nValueReferences: usize,
            values: *const f64,
            nValues: usize,
        ) -> fmi3Status {
            let ptr = instance as *mut #name;
        
            unsafe {
                let model: &mut #name = &mut *ptr;

                for i in 0..nValues {
                    let input_value_reference = *valueReferences.offset(i as isize) as usize;

                    match input_value_reference {
                        #(
                            #value_references => {
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