use syn;

/// Function that searches the fields of the struct and looks for a Option<FmuInfo> field.
pub fn search_for_fmu_info_field_name(input: &syn::DeriveInput) -> Option<syn::Ident>{
    let data: &syn::Data = &input.data;
    
    match *data {
        // Make sure the data is a struct
        syn::Data::Struct(ref data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    for field in fields.named.iter() {
                        match &field.ty {
                            syn::Type::Path(type_path) => {
                                let path = &type_path.path;

                                let segments = &path.segments;

                                let segment = &segments[0];

                                if segment.ident == "FmuInfo" {
                                    println!("Found FmuInfo field: {}", field.ident.as_ref().unwrap().to_string());
                                    return Some(field.ident.clone().unwrap());
                                }
                            },
                            _ => unimplemented!("A field in the struct seems to have an unsupported type"),
                        };
                    }
                },
                syn::Fields::Unnamed(_) | syn::Fields::Unit => unimplemented!("Only named fields are supported"),
            }
        },
        _ =>  panic!("Only structs are supported"),
    }

    None
}