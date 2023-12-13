use syn;

#[derive(Debug, Clone)]
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
    pub fn parse_fields(input: &syn::DeriveInput) -> Vec<Self> {
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
            syn::Data::Enum(_) | syn::Data::Union(_) =>  unimplemented!(),
        }
    }

    pub fn model_description_type(&self) -> String {
        match self.field_type.to_string().as_str() {
            "f64" => "Float64".to_string(),
            "bool" => "Boolean".to_string(),
            _ => unimplemented!(),
        }
    }
    
}
