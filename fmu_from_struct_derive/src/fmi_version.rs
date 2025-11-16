use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Enum that specifies the fmi version to use.
pub enum FmiVersion {
    Fmi2,
    Fmi3,
}

impl FmiVersion {
    /// Parses the fmi version from the input structure attributes.
    pub fn parse(input: &syn::DeriveInput) -> Self {
        let mut fmi_version = Self::Fmi3;
        
        let attributes = &input.attrs;

        for attribute in attributes {
            
            if attribute.path().is_ident("fmu_from_struct") {
                // Parse as Meta::List (attribute with parentheses)
                if let syn::Meta::List(meta_list) = &attribute.meta {
                    // Parse the nested content as a punctuated list of Meta items
                    let nested = meta_list.parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated
                    ).expect("Failed to parse fmu_from_struct arguments");
                    
                    // Iterate through nested meta items to find fmi_version
                    for meta in nested {
                        if let syn::Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("fmi_version") {
                                if let syn::Expr::Lit(lit) = &nv.value {
                                    if let syn::Lit::Int(int) = &lit.lit {
                                        let value = int.base10_parse::<usize>()
                                            .expect("Failed to parse fmi_version as integer");
                                        fmi_version = match value {
                                            2 => Self::Fmi2,
                                            3 => Self::Fmi3,
                                            _ => panic!("Only supports FMI version 2 and 3"),
                                        };
                                    } else {
                                        panic!("fmi_version must be an integer literal");
                                    }
                                } else {
                                    panic!("fmi_version must be a literal value");
                                }
                            }
                        }
                    }
                } else {
                    panic!("fmu_from_struct must be used as #[fmu_from_struct(...)]");
                }
            }
        }
        
        println!("Info: found FMI version {:?}", fmi_version);

        fmi_version
    }

    pub fn to_function_string(&self) -> String {
        match self {
            Self::Fmi2 => "fmi2".to_string(),
            Self::Fmi3 => "fmi3".to_string(),
        }
    }

    pub fn to_model_description_string(&self) -> String {
        match self {
            Self::Fmi2 => "2.0".to_string(),
            Self::Fmi3 => "3.0".to_string(),
        }
    }
    
    pub fn impl_fmi_version(&self) -> TokenStream2 {
        let function_name = match self {
            FmiVersion::Fmi2 => quote! { fmi2GetVersion },
            FmiVersion::Fmi3 => quote! { fmi3GetVersion },
        };
    
        let fmi_version_token = match self {
            FmiVersion::Fmi2 => quote! { "2.0\0" },
            FmiVersion::Fmi3 => quote! { "3.0\0" },
        };
    
        let version_tokens =  quote! {
            #[no_mangle]
            #[allow(non_snake_case)]
            pub extern "C" fn #function_name() -> *const ffi::c_char {
                #fmi_version_token.as_ptr() as *const ffi::c_char
            }
        };
    
        let types_platform_tokens = if *self == FmiVersion::Fmi2 {
            quote! {
                #[no_mangle]
                #[allow(non_snake_case)]
                pub extern "C" fn fmi2GetTypesPlatform() -> *const ffi::c_char {
                    "default\0".as_ptr() as *const ffi::c_char
                }
            }
        } else {
            quote! {}
        };
    
       quote! {
            #version_tokens
            #types_platform_tokens
        }
    }
    
    
}

