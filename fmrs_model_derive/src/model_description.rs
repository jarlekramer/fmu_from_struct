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

    write!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    write!(file, r#"
<fmiModelDescription 
    fmiVersion="{}" 
    modelName="{}" 
    description="to come"
    generationTool="The FmrsModel macro for automatic fmi generation for rust structs"
    {}="{{{}}}"
>
"#, fmi_version.to_model_description_string(), name, id_string, id)?;

    write!(file, r#"
<CoSimulation 
    modelIdentifier="{}" 
/>
"#, name)?;

// ------------------------- ModelVariables --------------------------------------------------------

    match fmi_version {
        FmiVersion::Fmi2 => {
            write!(file, r#"
<ModelVariables>
    <ScalarVariable name="time" valueReference="0" causality="independent" variability="continuous" description="Simulation time">
        <Real/>
    </ScalarVariable>
"#)?;  
        },
        FmiVersion::Fmi3 => { 
            write!(file, r#"
<ModelVariables>
    <Float64 name="time" valueReference="0" causality="independent" variability="continuous" description="Simulation time"/>
"#)?;
        }
    }

    for field in fields {
        match fmi_version {
            FmiVersion::Fmi2 => {
                write!(
                    file, 
                    "    <ScalarVariable name=\"{}\" valueReference=\"{}\" causality=\"{}\" variability=\"{}\" initial=\"exact\">\n",  
                    field.name,
                    field.value_reference,
                    field.causality.as_string(),
                    field.causality.variability_string(),
                )?;

                write!(
                    file,
                    "        <{} start=\"{}\"/>\n",
                    FieldInformation::get_fmi_type_name(fmi_version, &field.field_type),
                    FieldInformation::get_default_start_value_string(&field.field_type),
                )?;

                write!(
                    file,
                    "    </ScalarVariable>\n"
                )?;
            },
            FmiVersion::Fmi3 => {
                write!(
                    file, 
                    "    <{} name=\"{}\" valueReference=\"{}\" causality=\"{}\" variability=\"{}\" initial=\"exact\" start=\"{}\"/>\n", 
                    FieldInformation::get_fmi_type_name(fmi_version, &field.field_type), 
                    field.name,
                    field.value_reference,
                    field.causality.as_string(),
                    field.causality.variability_string(),
                    FieldInformation::get_default_start_value_string(&field.field_type),
                )?;
            },
        }
        
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