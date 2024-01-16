#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FmiVersion {
    Fmi2,
    Fmi3,
}

impl FmiVersion {
    pub fn parse(input: &syn::DeriveInput) -> Self {
        let mut fmi_version = Self::Fmi3;
        
        let attributes = &input.attrs;

        for attribute in attributes {
            let name = &attribute.path().segments[0].ident.to_string();
            
            if name == "fmi_version" {
                let meta = &attribute.meta;

                match meta {
                    syn::Meta::NameValue(meta_name_value) => {
                        let value = &meta_name_value.value;

                        match value {
                            syn::Expr::Lit(lit) => {
                                match &lit.lit {
                                    syn::Lit::Int(int) => {
                                        let value = int.base10_parse::<usize>().unwrap();

                                        match value {
                                            2 => fmi_version = Self::Fmi2,
                                            3 => fmi_version = Self::Fmi3,
                                            _ => panic!("Only supports FMI version 2 and 3"),
                                        }
                                    },
                                    _ => panic!("Only supports integer value"),
                                }
                            },
                            _ => panic!("Only supports literal value"),
                        }
                    },
                    _ => panic!("Only supports name value"),
                }
            }
        }

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
}