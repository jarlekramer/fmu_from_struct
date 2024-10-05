//! Module to analyze the fields in the strut in the context of the FMI standard.
//! 
//! The purpose is to extract fields that should be exposed as an FMI variable, and to determine
//! what type of variable it is. The type is both the data type and causality, i.e., input, output, 
//! or parameter.

use syn::{self, Expr};

use crate::fmi_version::FmiVersion;
pub mod causality;

use causality::Causality;

#[derive(Debug, Clone)]
/// This struct stores the relevant information about a field
pub struct FieldInformation {
    /// The name of the field, taken directly from the input struct
    pub name: syn::Ident,
    /// The data type of the field, taken directly from the input struct
    pub field_type: syn::Ident,
    /// The causality of the field, taken from the last attribute specifying the causality
    pub causality: Causality,
    /// The value reference of the field, used to uniquely identify the field in the setters, 
    /// getters, and the model description
    pub value_reference: usize,
    /// Default value of the field
    pub default_value: Option<String>,
}

impl FieldInformation {
    /// Parsers the fields in the struct and converts to a vector of FieldInformation.
    /// 
    /// This is to ease the processing of the struct later on.
    pub fn parse(input: &syn::DeriveInput) -> Vec<Self> {
        let data: &syn::Data = &input.data; 

        match *data {
            // Make sure the data is a struct
            syn::Data::Struct(ref data) => {
                match data.fields {
                    syn::Fields::Named(ref fields) => Self::parse_named_fields(fields),
                    syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!("Only named fields are supported"),
                }
            },
            _ =>  panic!("Only structs are supported"),
        }
    }


    /// Parses the named fields of the struct
    fn parse_named_fields(fields: &syn::FieldsNamed) -> Vec<FieldInformation> {
        // The value reference used to identify the fields in the model description, and set and get 
        // functions
        let mut value_reference = 1; 
    
        let mut fields_information: Vec<FieldInformation> = Vec::new();

        let mut causality = Causality::Parameter;

        for field in fields.named.iter() {
            // Check for updates to the variable type
            causality.update(&field.attrs);

            // Only add the field if it is public
            if let syn::Visibility::Public(_) = &field.vis {
                let field_type = Self::parse_field_type(field);

                let default_value = Self::check_for_default_value(&field.attrs);

                let field_information = FieldInformation {
                    name: field.ident.clone().unwrap(),
                    field_type,
                    causality: causality.clone(),
                    value_reference,
                    default_value: default_value,
                };

                fields_information.push(field_information);

                value_reference += 1;
            }
        }
        
        fields_information
    }

    /// Parses the field type from the field. Only supports 'Path' type, which includes the basic
    /// data types in Rust.
    fn parse_field_type(field: &syn::Field) -> syn::Ident {
        match &field.ty {
            syn::Type::Path(type_path) => {
                let path = &type_path.path;

                let segments = &path.segments;

                let segment = &segments[0];

                segment.ident.clone()
            },
            _ => unimplemented!("Only 'Path' is supported"),
        }
    }

    /// Filters the fields based on the data type. Used to get all the fields of a certain type in
    /// the setter and getter functions
    pub fn filter_on_type(fields: &[Self], field_type: &syn::Ident) -> Vec<Self> {
        fields.iter()
            .filter(|field| field.field_type == *field_type)
            .map(|field| field.clone())
            .collect()
    }

    pub fn check_for_default_value(attributes: &[syn::Attribute]) -> Option<String> {
        for attribute in attributes.iter() {
            let attribute_type = &attribute.path().segments[0].ident.to_string();
            
            if attribute_type == "fmu_default" {
                let expression: syn::Expr = attribute.parse_args().unwrap();

                let expression_lit = match expression {
                    Expr::Lit(lit) => lit,
                    _ => unimplemented!("Only literals are supported"),
                };

                let lit = match expression_lit.lit {
                    syn::Lit::Str(lit) => lit.value(),
                    syn::Lit::Float(lit) => format!("{}", lit),
                    _ => unimplemented!("Only string literals are supported"),
                };

                return Some(lit);
            }
        }

        None
    }

    /// Generates the model description string for the field. This is used to write the model 
    /// description file.
    pub fn model_description_string(&self, fmi_version: FmiVersion) -> String {
        let variable_start_kw = match fmi_version {
            FmiVersion::Fmi2 => "ScalarVariable".to_string(),
            FmiVersion::Fmi3 => FieldInformation::get_fmi_type_name(fmi_version, &self.field_type),
        };

        let tail = format!(
            "    </{}>\n",
            variable_start_kw,
        );
        
        let header = format!(
            "    <{} name=\"{}\" valueReference=\"{}\" causality=\"{}\" variability=\"{}\">\n",
            variable_start_kw,
            self.name,
            self.value_reference,
            self.causality.as_string(),
            self.causality.variability_string(),
        );

        let start_value_name = match self.field_type.to_string().as_str() {
            "f64" => "Real".to_string(),
            "bool" => "Boolean".to_string(),
            "i32" => "Real".to_string(),
            "String" => "String".to_string(),
            _ => unimplemented!("A start value for this type is not implemented: {}", self.field_type.to_string()),
        };

        let body = match self.causality {
            Causality::Parameter | Causality::Input => {
                format!(
                    "        <{} start=\"{}\"/>\n",
                    start_value_name,
                    self.get_default_start_value_string(),
                )
            },
            Causality::Output => {
                format!("        <{}/>\n", start_value_name)
            }
        };

        format!("{}{}{}", header, body, tail)
    }

    /// Converts the name of the rust variable to the fmi name. Depends both on the FMI version and
    /// the data type.
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
            "String" => "String".to_string(),
            _ => panic!(
                "From field_information.rs, get_fmi_type_name. Type not supported: {}", 
                field_type.to_string()
            ),
        }
    }

    pub fn get_default_start_value_string(&self) -> String {
        match &self.default_value {
            Some(value) => value.clone(),
            None => {
                match self.field_type.to_string().as_str() {
                    "f64" => "0.0".to_string(),
                    "bool" => "false".to_string(),
                    "i32" => "0".to_string(),
                    "String" => "".to_string(),
                    _ => unimplemented!(
                        "A default start value for this type is not implemented: {}", 
                        self.field_type.to_string()
                    ),
                }
            },
        }
       
    }
}
