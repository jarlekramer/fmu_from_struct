use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::quote;
use syn;

mod model_description;
mod field_information;

use field_information::FieldInformation;

use fmi_types::*;

#[proc_macro_derive(FmrsModel, attributes(parameter, input, output))]
pub fn fmrs_model_derive(input: TokenStream) -> TokenStream { 
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;
    let name_string = name.to_string();

    let fields = FieldInformation::parse_fields(&input);

    let _write_res = model_description::generate_model_description(&name_string, &fields);

    let get_tokens = impl_get_functions(name, &fields);
    let div_tokens = impl_fmrs_model(&input);

    dbg!(get_tokens.to_string());

    quote! {
        #get_tokens
        #div_tokens
    }.into()
}

fn impl_get_functions(name: &syn::Ident, fields: &Vec<FieldInformation>) -> TokenStream2 {
    let field_names      = fields.iter().map(|field| &field.name);
    let value_references = fields.iter().map(|field| field.value_reference);
    
    let tokens = quote! {
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
                    let input_value_reference = valueReferences.offset(i as isize) as usize;

                    match input_value_reference {
                        #(
                            #value_references => {
                                let value = &model.#field_names;
                                *values.offset(i as isize) = *value;
                            }
                        )*
                        _ => {}
                    }
                }

            }
            
            fmi3Status::fmi3OK
        }
    };

    tokens
}


fn impl_fmrs_model(input: &syn::DeriveInput) -> TokenStream2 {
    let tokens = quote!();

    tokens
}