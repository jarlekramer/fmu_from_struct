//! This module contains the function to generate the model description file based on the fields of
//! the struct.

use std::fs::File;
use std::io::{Write, Error};

use uuid::Uuid;

use super::field_information::{FieldInformation, Causality};
use super::fmi_version::FmiVersion;

/// Parses the struct and writes a fmi model description based on the variables.
pub fn generate_model_description(fmi_version: FmiVersion, name: &str, fields: &Vec<FieldInformation>) -> Result<(), Error>  {
    let id = Uuid::new_v4();

    let path = "modelDescription.xml";

    let mut file = File::create(path)?;

    let id_string = match fmi_version {
        FmiVersion::Fmi2 => "guid".to_string(),
        FmiVersion::Fmi3 => "instantiationToken".to_string(),
    };

    // ------------------------- Header ------------------------------------------------------------

    write!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    write!(file, "<fmiModelDescription\n")?;
    write!(file, "    fmiVersion=\"{}\"\n", fmi_version.to_model_description_string())?;
    write!(file, "    modelName=\"{}\"\n", name)?;
    write!(file, "    description=\"to come\"\n")?;
    write!(file, "    generationTool=\"The FmrsModel macro for automatic fmi generation for rust structs\"\n")?;
    write!(file, "    {}=\"{}\"\n", id_string, id)?;
    write!(file, ">\n")?;

    write!(file, "<CoSimulation\n    modelIdentifier=\"{}\"/>\n", name)?;

    // ------------------------- ModelVariables ----------------------------------------------------

    write!(file, "<ModelVariables>\n")?;

    for field in fields {
        write!(file, "{}", field.model_description_string(fmi_version))?;
    }

    write!(file, "</ModelVariables>\n\n")?;


    // ------------------------- ModelStructure ----------------------------------------------------
    write!(file, "<ModelStructure>\n")?;
    let outputs: Vec<usize> = fields.iter()
        .filter(|field| field.causality == Causality::Output)
        .map(|field| field.value_reference).collect();

    match fmi_version {
        FmiVersion::Fmi2 => {
            write!(file, "    <Outputs>\n")?;
            for vr in outputs {
                write!(
                    file, 
                    "        <Unknown index=\"{}\"/>\n", 
                    vr,
                )?;
            }
            write!(file, "    </Outputs>\n")?;
        },
        FmiVersion::Fmi3 => {
            for vr in outputs {
                write!(
                    file, 
                    "        <Output valueReference=\"{}\"/>\n", 
                    vr,
                )?;
            }
        }
    }

    write!(file, "</ModelStructure>\n\n")?;
    
    // ------------------------- End ---------------------------------------------------------------

    write!(file, "</fmiModelDescription>\n")?;

    Ok(())
}