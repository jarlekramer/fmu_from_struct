use syn;

use crate::fmi_version::FmiVersion;

#[derive(Debug, Clone, PartialEq)]
pub enum Causality {
    Parameter,
    Input,
    Output,
}

impl Causality {
    pub fn from_string(string: &str) -> Self {
        match string {
            "parameter" => Causality::Parameter,
            "input"     => Causality::Input,
            "output"    => Causality::Output,
            _ => unimplemented!(),
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            Causality::Parameter => "parameter".to_string(),
            Causality::Input => "input".to_string(),
            Causality::Output => "output".to_string(),
        }
    }

    pub fn variability_string(&self) -> String {
        match self {
            Causality::Parameter => "tunable".to_string(),
            Causality::Input => "continuous".to_string(),
            Causality::Output => "continuous".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldInformation {
    pub name: syn::Ident,
    pub field_type: syn::Ident,
    pub causality: Causality,
    pub value_reference: usize,
}

impl FieldInformation {
    /// Parsers the fields in the struct and converts to a vector of FieldInformation.
    /// 
    /// This is to easy the processing of the struct later on.
    pub fn parse(input: &syn::DeriveInput) -> Vec<Self> {
        let data: &syn::Data = &input.data;

        match *data {
            syn::Data::Struct(ref data) => {
                match data.fields {
                    syn::Fields::Named(ref fields) => {
                        let mut value_reference = 1;
    
                        let mut fields_information: Vec<FieldInformation> = Vec::new();

                        let mut causality = Causality::Parameter;
    
                        for field in fields.named.iter() {
                            let visibility = &field.vis;

                            let attributes = &field.attrs;

                            if attributes.len() > 0 {
                                let attribute = &attributes[0]; // TODO: consider the need to support multiple attributes

                                let path = &attribute.path();

                                let segments = &path.segments;

                                let segment = &segments[0];

                                let ident = &segment.ident;

                                let string = ident.to_string();

                                causality = Causality::from_string(&string);
                            }

                            // Skip private fields
                            if let syn::Visibility::Public(_) = visibility {
                                let field_type = match &field.ty {
                                    syn::Type::Path(type_path) => {
                                        let path = &type_path.path;
        
                                        let segments = &path.segments;
        
                                        let segment = &segments[0];
        
                                        segment.ident.clone()
                                    },
                                    _ => unimplemented!(),
                                };

                                let field_information = FieldInformation {
                                    name: field.ident.clone().unwrap(),
                                    field_type,
                                    causality: causality.clone(),
                                    value_reference,
                                };
        
                                fields_information.push(field_information);
        
                                value_reference += 1;
                            }
                        }
                       
                        fields_information
                    },
                    syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!(),
                }
            },
            _ =>  panic!("Only structs are supported"),
        }
    }

    pub fn filter_on_type(fields: &[Self], field_type: &syn::Ident) -> Vec<Self> {
        fields.iter()
            .filter(|field| field.field_type == *field_type)
            .map(|field| field.clone())
            .collect()
    }

    pub fn get_fmi_type_name(fmi_version: FmiVersion, field_type: &syn::Ident) -> String {
        match field_type.to_string().as_str() {
            "f64" => {
                match fmi_version {
                    FmiVersion::Fmi3 => "Float64".to_string(),
                    FmiVersion::Fmi2 => "Real".to_string(),
                }
            },
            "f32" => {
                match fmi_version {
                    FmiVersion::Fmi3 => "Float32".to_string(),
                    FmiVersion::Fmi2 => panic!("FMI 2 does not support f32"),
                }
            },
            "i8" => match fmi_version {
                FmiVersion::Fmi3 => "Int8".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support i8"),
            },
            "i16" => match fmi_version {
                FmiVersion::Fmi3 => "Int16".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support i16"),
            },
            "i32" => match fmi_version {
                FmiVersion::Fmi3 => "Int32".to_string(),
                FmiVersion::Fmi2 => "Integer".to_string(),
            },
            "i64" => match fmi_version {
                FmiVersion::Fmi3 => "Int64".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support i64"),
            },
            "u8" => match fmi_version {
                FmiVersion::Fmi3 => "UInt8".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support u8"),
            },
            "u16" => match fmi_version {
                FmiVersion::Fmi3 => "UInt16".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support u16"),
            },
            "u32" => match fmi_version {
                FmiVersion::Fmi3 => "UInt32".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support u32"),
            },
            "u64" => match fmi_version {
                FmiVersion::Fmi3 => "UInt64".to_string(),
                FmiVersion::Fmi2 => panic!("FMI 2 does not support u64"),
            },
            "bool" => "Boolean".to_string(),
            _ => panic!("Type not supported"),
        }
    }

    pub fn get_default_start_value_string(field_type: &syn::Ident) -> String {
        match field_type.to_string().as_str() {
            "f64" => "0.0".to_string(),
            "bool" => "false".to_string(),
            "i32" => "0".to_string(),
            _ => unimplemented!(),
        }
    }
    
}
