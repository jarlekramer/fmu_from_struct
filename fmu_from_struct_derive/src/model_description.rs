//! This module contains the function to generate the model description file based on the fields of
//! the struct.

use std::env;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;

use uuid::Uuid;

use super::field_information::{FieldInformation, Causality};
use super::fmi_version::FmiVersion;

/// Environment variable that the `cargo fmu-from-struct` subcommand uses to tell the macro where
/// the target directory is.
const TARGET_DIR_ENV_VAR: &str = "FMU_FROM_STRUCT_TARGET_DIR";

/// Returns the path where the model description is written: 
/// `<target_dir>/fmu/<crate_name>/modelDescription.xml`.
/// 
/// A proc macro is not told where the target directory is, so it is found in the following order:
/// 1. The `FMU_FROM_STRUCT_TARGET_DIR` environment variable, set by `cargo fmu-from-struct`
/// 2. The `CARGO_TARGET_DIR` environment variable, if set by the user
/// 3. The `target` folder in the current working directory. Cargo runs the compiler from the
///    workspace root (or the crate folder, for crates outside a workspace), so this is the default
///    target directory.
pub fn model_description_path() -> PathBuf {
    let current_dir = env::current_dir().unwrap_or_default();

    let target_dir = env::var_os(TARGET_DIR_ENV_VAR)
        .or_else(|| env::var_os("CARGO_TARGET_DIR"))
        .map(|dir| current_dir.join(dir))
        .unwrap_or_else(|| current_dir.join("target"));

    let crate_name = env::var("CARGO_CRATE_NAME").unwrap_or_else(|_| "unknown_crate".to_string());

    target_dir.join("fmu").join(crate_name).join("modelDescription.xml")
}

/// Marker placed in front of the model description that is embedded in the compiled library. The
/// `cargo fmu-from-struct` subcommand searches for this marker to extract the model description.
/// Must match the marker used in `cargo-fmu-from-struct`.
const EMBEDDED_MODEL_DESCRIPTION_MARKER: &str = "fmu_from_struct::model_description::begin\n";

/// Writes the model description to the file given by `model_description_path`, so that it can be
/// inspected by the user.
pub fn write_model_description(model_description: &str) -> std::io::Result<()> {
    let path = model_description_path();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, model_description)
}

/// Generates a static that embeds the model description in the compiled library, so that the FMU
/// can be packaged with a model description that is guaranteed to match the library. Cargo does
/// not recompile a crate that is up to date, so the model description file can be from a build
/// with a different profile or different features than the library.
///
/// The static is exported, so that the linker does not remove it. The content is the marker, the
/// model description, and a NULL byte to mark the end.
pub fn impl_embedded_model_description(model_description: &str) -> TokenStream2 {
    let mut bytes: Vec<u8> = EMBEDDED_MODEL_DESCRIPTION_MARKER.as_bytes().to_vec();
    bytes.extend_from_slice(model_description.as_bytes());
    bytes.push(0);

    let length = bytes.len();
    let byte_string = Literal::byte_string(&bytes);

    quote! {
        #[no_mangle]
        #[doc(hidden)]
        pub static FMU_FROM_STRUCT_EMBEDDED_MODEL_DESCRIPTION: [u8; #length] = *#byte_string;
    }
}

/// Generates a fmi model description based on the variables in the struct.
pub fn generate_model_description(
    fmi_version: FmiVersion,
    name: &str,
    fields: &Vec<FieldInformation>
) -> String {
    let id = Uuid::new_v4();

    let mut xml = String::new();

    let id_string = match fmi_version {
        FmiVersion::Fmi2 => "guid".to_string(),
        FmiVersion::Fmi3 => "instantiationToken".to_string(),
    };

    // ------------------------- Header ------------------------------------------------------------

    write!(xml, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").unwrap();
    write!(xml, "<fmiModelDescription\n").unwrap();
    write!(xml, "    fmiVersion=\"{}\"\n", fmi_version.to_model_description_string()).unwrap();
    write!(xml, "    modelName=\"{}\"\n", name).unwrap();
    write!(xml, "    description=\"to come\"\n").unwrap();
    write!(xml, "    generationTool=\"The fmu_from_struct macro for automatic fmi generation for rust structs\"\n").unwrap();
    write!(xml, "    {}=\"{}\"\n", id_string, id).unwrap();
    write!(xml, ">\n").unwrap();

    // The do_step function accepts any time step, so variable step sizes are supported. The
    // standard defaults to false if the attribute is not set.
    write!(
        xml,
        "<CoSimulation\n    modelIdentifier=\"{}\"\n    canHandleVariableCommunicationStepSize=\"true\"/>\n",
        name
    ).unwrap();

    // ------------------------- ModelVariables ----------------------------------------------------

    write!(xml, "<ModelVariables>\n").unwrap();

    if fmi_version == FmiVersion::Fmi3 {
        write!(xml, "    <Float64 name=\"time\" valueReference=\"0\" causality=\"independent\" variability=\"continuous\"/>\n").unwrap();
    }

    for field in fields {
        write!(xml, "{}", field.model_description_string(fmi_version)).unwrap();
    }

    write!(xml, "</ModelVariables>\n\n").unwrap();


    // ------------------------- ModelStructure ----------------------------------------------------
    write!(xml, "<ModelStructure>\n").unwrap();
    let outputs: Vec<usize> = fields.iter()
        .filter(|field| field.causality == Causality::Output)
        .map(|field| field.value_reference).collect();

    match fmi_version {
        FmiVersion::Fmi2 => {
            write!(xml, "    <Outputs>\n").unwrap();
            for vr in &outputs {
                write!(
                    xml,
                    "        <Unknown index=\"{}\"/>\n",
                    vr,
                ).unwrap();
            }
            write!(xml, "    </Outputs>\n").unwrap();

            write!(xml, "   <InitialUnknowns>\n").unwrap();
            for vr in &outputs {
                write!(
                    xml,
                    "        <Unknown index=\"{}\"/>\n",
                    vr,
                ).unwrap();
            }
            write!(xml, "   </InitialUnknowns>\n").unwrap();
        },
        FmiVersion::Fmi3 => {
            for vr in &outputs {
                write!(
                    xml,
                    "        <Output valueReference=\"{}\"/>\n",
                    vr,
                ).unwrap();
            }

            for vr in &outputs {
                write!(
                    xml,
                    "        <InitialUnknown valueReference=\"{}\"/>\n",
                    vr,
                ).unwrap();
            }
        }
    }

    write!(xml, "</ModelStructure>\n\n").unwrap();

    // ------------------------- End ---------------------------------------------------------------

    write!(xml, "</fmiModelDescription>\n").unwrap();

    xml
}
