#[derive(Debug, Clone, PartialEq)]
/// Enum defining the possible causalities in the FMI standard.
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
            Causality::Input     => "input".to_string(),
            Causality::Output    => "output".to_string(),
        }
    }

    pub fn variability_string(&self) -> String {
        match self {
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

    pub fn update(&mut self, attributes: &[syn::Attribute]) {
        for attribute in attributes.iter() {
            let attribute_type = &attribute.path().segments[0].ident.to_string();
            
            if attribute_type == "parameter" || attribute_type == "input" || attribute_type == "output" {
                *self = Causality::from_string(attribute_type);
            }
        }
    }
}