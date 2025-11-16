//! Module to analyze the fields in the struct in the context of the FMI standard.
//!
//! The purpose is to extract fields that should be exposed as an FMI variable, and to determine
//! what type of variable it is. The type is both the data type and causality, i.e., input, output,
//! or parameter.

use syn;

use crate::fmi_version::FmiVersion;

#[derive(Debug, Clone, PartialEq)]
/// Enum defining the possible causalities in the FMI standard.
pub enum Causality {
    /// Variable that do not change during simulation
    Parameter,
    /// Variables that are set before each time step and then used during doStep
    Input,
    /// Variables that are updated during doStep and can be read after each time step
    Output,
}

impl Causality {
    pub fn option_from_string(string: &str) -> Option<Self> {
        match string {
            "parameter" => Some(Causality::Parameter),
            "input"     => Some(Causality::Input),
            "output"    => Some(Causality::Output),
            _ => None,
        }
    }
    pub fn as_string(&self) -> String {
        match self {
            Causality::Parameter => "parameter".to_string(),
            Causality::Input     => "input".to_string(),
            Causality::Output    => "output".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
/// This struct stores the relevant information about a field. This needs to be populated for every
/// public variable in the struct that uses the macro. The information is later used to generate 
/// both the model description and the setter and getter functions.
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
    /// The default value
    pub start_value_string: Option<String>,
}

impl FieldInformation {
    /// Parsers the fields in the struct and converts to a vector of FieldInformation. The function
    /// mainly checks that the input is a struct with named fields, and then calls the main parsing
    /// function, parse_named_fields.
    pub fn parse(input: &syn::DeriveInput) -> Vec<Self> {
        let data: &syn::Data = &input.data;

        match *data {
            // Make sure the data is a struct
            syn::Data::Struct(ref data) => {
                // Make sure the fields are named
                match data.fields {
                    syn::Fields::Named(ref fields) => {
                        Self::parse_named_fields(fields)
                    },
                    syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                        unimplemented!("Only named fields are supported")
                    },
                }
            },
            _ =>  panic!("Only structs are supported"),
        }
    }
    
    /// Generates a vector of Self by parsing named fields in the struct. This function is where
    /// the main parsing logic is implemented.
    pub fn parse_named_fields(fields: &syn::FieldsNamed) -> Vec<Self> {
        // Tracker for the value reference. Starts at 1 and then increments for each public field.
        let mut value_reference = 1;

        let mut fields_information: Vec<FieldInformation> = Vec::new();
        
        // Default causality is parameter
        let mut causality = Causality::Parameter;

        for field in fields.named.iter() {
            // Check the attributes for a field
            let attributes = &field.attrs;
            
            let start_value_string = Self::check_for_start_value_string(attributes);
            let potential_new_causality = Self::check_for_new_causality(attributes);
            
            if let Some(new_causality) = potential_new_causality {
                causality = new_causality;
            }

            // Skip private fields
            let visibility = &field.vis;

            if let syn::Visibility::Public(_) = visibility {
                let field_type = match &field.ty {
                    syn::Type::Path(type_path) => {
                        let path = &type_path.path;

                        let segments = &path.segments;

                        let segment = &segments[0];

                        segment.ident.clone()
                    },
                    _ => unimplemented!("A field in the struct seems to have an unsupported type"),
                };

                // Skipping FmuInfo field
                if field_type == "FmuInfo" {
                    continue;
                }

                let field_information = FieldInformation {
                    name: field.ident.clone().unwrap(),
                    field_type,
                    causality: causality.clone(),
                    value_reference,
                    start_value_string,
                };

                fields_information.push(field_information);

                value_reference += 1;
            }
        }

        fields_information
    }
    
    /// Checks whether the causality is changed in any of the attributes
    pub fn check_for_new_causality(attributes: &[syn::Attribute]) -> Option<Causality> {
        for attr in attributes.iter() {
            // Check that the attribute is in the right group
            if !attr.path().is_ident("fmu_from_struct") {
                continue;
            }
            
            if let syn::Meta::List(meta_list) = &attr.meta {
                if let Ok(ident) = meta_list.parse_args::<syn::Ident>() {
                    return Causality::option_from_string(ident.to_string().as_str());
                }
            }
        }
        
        None
    }
    
    /// Check for default function in the attributes and return it as 
    pub fn check_for_start_value_string(attributes: &[syn::Attribute]) -> Option<String> {
        for attr in attributes.iter() {
            // Check that the attribute is in the right group
            if !attr.path().is_ident("fmu_from_struct") {
                continue;
            }
        
            match &attr.meta {
                syn::Meta::List(meta_list) => {
                    if let Ok(meta_name_value) = meta_list.parse_args::<syn::MetaNameValue>() {
                        // Check if the attribute type is the right one
                        if !meta_name_value.path.is_ident("start_value") {
                            continue
                        }
                        
                        // Extract the function name
                        match &meta_name_value.value {
                            syn::Expr::Lit(expr_lit) => {
                                match &expr_lit.lit {
                                    syn::Lit::Str(string_value) => {
                                        return Some(string_value.value());
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }   
                    }
                },
                _ => {}
            }
        }
            
        None
    }

    /// Filters the fields based on the data type. Used to get all the fields of a certain type in
    /// the setter and getter functions
    pub fn filter_on_type(fields: &[Self], field_type: &syn::Ident) -> Vec<Self> {
        fields.iter()
            .filter(|field| field.field_type == *field_type)
            .map(|field| field.clone())
            .collect()
    }

    pub fn variability_string(&self) -> String {
        match self.causality {
            Causality::Parameter => {
                "fixed".to_string()
            },
            Causality::Input => {
                "discrete".to_string()
            },
            Causality::Output => {
                "discrete".to_string()
            },
        }
    }
    
    /// Generates the model description string for a field
    pub fn model_description_string(&self, fmi_version: FmiVersion) -> String {
        let variable_start_kw = match fmi_version {
            FmiVersion::Fmi2 => "ScalarVariable".to_string(),
            FmiVersion::Fmi3 => FieldInformation::get_fmi_type_name(fmi_version, &self.field_type),
        };

        let tail = format!(
            "    </{}>\n",
            variable_start_kw,
        );

        match fmi_version {
            FmiVersion::Fmi2 => {
                let header = format!(
                    "    <{} name=\"{}\" valueReference=\"{}\" causality=\"{}\" variability=\"{}\">\n",
                    variable_start_kw,
                    self.name,
                    self.value_reference,
                    self.causality.as_string(),
                    self.variability_string(),
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
                            self.get_start_value_string(),
                        )
                    },
                    Causality::Output => {
                        format!("        <{}/>\n", start_value_name)
                    }
                };

                format!("{}{}{}", header, body, tail)
            },
            FmiVersion::Fmi3 => {
                let start = format!(
                    "    <{} name=\"{}\" valueReference=\"{}\" causality=\"{}\" variability=\"{}\"",
                    variable_start_kw,
                    self.name,
                    self.value_reference,
                    self.causality.as_string(),
                    self.variability_string()
                );

                let end = match self.causality {
                    Causality::Parameter | Causality::Input => {
                        format!(
                            " start=\"{}\"/>\n",
                            self.get_start_value_string(),
                        )
                    },
                    Causality::Output => {
                        "/>\n".to_string()
                    }
                };

                format!("{}{}", start, end)
            }
        }

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

    pub fn get_start_value_string(&self) -> String {
        if let Some(start_value_string) = &self.start_value_string {
            start_value_string.clone()
        } else {
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
        }
        
        
    }
}
