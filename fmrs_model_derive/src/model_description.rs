use std::fs::File;
use std::io::{Write, Error};

use uuid::Uuid;

use super::field_information::{FieldInformation, Causality};

/// Parses the struct and writes a fmi model description based on the variables.
pub fn generate_model_description(name: &str, fields: &Vec<FieldInformation>) -> Result<(), Error>  {
    let id = Uuid::new_v4();

    let path = "modelDescription.xml";

    let mut file = File::create(path)?;

    write!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    write!(file, r#"
<fmiModelDescription 
    fmiVersion="3.0" 
    modelName="{}" 
    description="to come"
    generationTool="The FmrsModel macro for automatic fmi generation for rust structs"
    instantiationToken="{{{}}}"
>
"#, name, id)?;

    write!(file, r#"
<CoSimulation 
    modelIdentifier="{}" 
/>
"#, name)?;

// ------------------------- ModelVariables --------------------------------------------------------

    write!(file, r#"
<ModelVariables>
    <Float64 name="time" valueReference="0" causality="independent" variability="continuous" description="Simulation time"/>
"#)?;

    for field in fields {
        write!(
            file, 
            "    <{} name=\"{}\" valueReference=\"{}\" causality=\"{}\" variability=\"{}\" initial=\"exact\" start=\"{}\"/>\n", 
            field.model_description_type(), 
            field.name,
            field.value_reference,
            field.causality.as_string(),
            field.causality.variability_string(),
            field.default_start_value_string(),
        )?;
    }

    write!(file, "</ModelVariables>\n\n")?;


    // ------------------------- ModelStructure ----------------------------------------------------

    write!(file, "<ModelStructure>\n")?;

    for field in fields {
        if let Causality::Output = field.causality {
            write!(
                file, 
                "    <Output valueReference=\"{}\"/>\n", 
                field.value_reference,
            )?;
        } else if let Causality::Input = field.causality {
            write!(
                file, 
                "    <Input valueReference=\"{}\"/>\n", 
                field.value_reference,
            )?;
        }
    }

    write!(file, "</ModelStructure>\n\n")?;

    // ------------------------- End ---------------------------------------------------------------

    write!(file, "</fmiModelDescription>\n")?;

    Ok(())
}