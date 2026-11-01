use std::fs;
use std::path::Path;

use xmltree::Element;

use crate::error::CliError;

/// Marker placed in front of the model description that the macro embeds in the compiled library.
/// Must match the marker used in `fmu_from_struct_derive`.
const EMBEDDED_MODEL_DESCRIPTION_MARKER: &[u8] = b"fmu_from_struct::model_description::begin\n";

/// The model description, together with the information from it that is needed to package the FMU
pub struct ModelDescription {
    /// The full model description file content
    pub xml: String,
    pub model_name: String,
    pub model_identifier: String,
    /// The major FMI version, i.e., 2 or 3
    pub fmi_major_version: u32,
}

impl ModelDescription {
    /// Extracts the model description that is embedded in a compiled library by the macro. Returns
    /// `None` if the library does not contain a model description, i.e., it is not an FMU made
    /// with fmu_from_struct (version 0.3 or later).
    ///
    /// The model description is read from the library, rather than from the model description
    /// file, as the file can be from a build with another profile or other features.
    pub fn from_library(library_path: &Path) -> Result<Option<Self>, CliError> {
        let bytes = fs::read(library_path)?;

        let start = match find(&bytes, EMBEDDED_MODEL_DESCRIPTION_MARKER) {
            Some(index) => index + EMBEDDED_MODEL_DESCRIPTION_MARKER.len(),
            None => return Ok(None),
        };

        let length = bytes[start..].iter().position(|&byte| byte == 0).ok_or_else(|| {
            CliError::new(format!(
                "The embedded model description in {} is not terminated", library_path.display()
            ))
        })?;

        let xml = String::from_utf8(bytes[start..start + length].to_vec()).map_err(|_| {
            CliError::new(format!(
                "The embedded model description in {} is not valid UTF-8", library_path.display()
            ))
        })?;

        Self::parse(xml).map(Some)
    }

    fn parse(xml: String) -> Result<Self, CliError> {
        let root = Element::parse(xml.as_bytes())?;

        let attribute = |name: &str| root.attributes.get(name).cloned().ok_or_else(|| {
            CliError::new(format!("{} not found in the model description", name))
        });

        let model_name = attribute("modelName")?;

        let fmi_version = attribute("fmiVersion")?;

        let fmi_major_version: u32 = fmi_version.split('.').next()
            .and_then(|major| major.parse().ok())
            .ok_or_else(|| CliError::new(format!("Invalid fmiVersion: {}", fmi_version)))?;

        let model_identifier = root.get_child("CoSimulation")
            .and_then(|co_simulation| co_simulation.attributes.get("modelIdentifier").cloned())
            .ok_or_else(|| CliError::new(
                "CoSimulation modelIdentifier not found in the model description"
            ))?;

        Ok(Self {
            xml,
            model_name,
            model_identifier,
            fmi_major_version,
        })
    }
}

/// Returns the index of the first occurrence of `pattern` in `bytes`
fn find(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    bytes.windows(pattern.len()).position(|window| window == pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_from_bytes() {
        let xml = "<?xml version=\"1.0\"?>\n<fmiModelDescription fmiVersion=\"2.0\" \
            modelName=\"Test\"><CoSimulation modelIdentifier=\"Test\"/></fmiModelDescription>\n";

        let mut library: Vec<u8> = b"some binary data\0\x01\x02".to_vec();
        library.extend_from_slice(EMBEDDED_MODEL_DESCRIPTION_MARKER);
        library.extend_from_slice(xml.as_bytes());
        library.extend_from_slice(b"\0more binary data");

        let path = std::env::temp_dir().join("cargo_fmu_from_struct_test_library.bin");
        fs::write(&path, &library).unwrap();

        let model_description = ModelDescription::from_library(&path).unwrap().unwrap();

        assert_eq!(model_description.xml, xml);
        assert_eq!(model_description.model_name, "Test");
        assert_eq!(model_description.model_identifier, "Test");
        assert_eq!(model_description.fmi_major_version, 2);

        fs::write(&path, b"no model description here").unwrap();
        assert!(ModelDescription::from_library(&path).unwrap().is_none());

        fs::remove_file(&path).ok();
    }
}
