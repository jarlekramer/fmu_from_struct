//! Writes the FMU zip file

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::CliError;
use crate::model_description::ModelDescription;
use crate::platform::Platform;

/// The files that goes into the FMU
pub struct FmuContent<'a> {
    pub model_description: &'a ModelDescription,
    pub library_path: PathBuf,
    /// Folder to copy into the 'resources' folder. An empty folder is added if `None`
    pub resources: Option<PathBuf>,
}

/// Writes the FMU to the output path. The structure of the FMU is:
/// - modelDescription.xml
/// - binaries/<platform>/<modelIdentifier>.<dll|so|dylib>
/// - resources/...
pub fn write_fmu(content: &FmuContent, platform: &Platform, output: &Path) -> Result<(), CliError> {
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut zip = ZipWriter::new(File::create(output)?);

    let options = SimpleFileOptions::default().unix_permissions(0o755);

    zip.start_file("modelDescription.xml", options)?;
    zip.write_all(content.model_description.xml.as_bytes())?;

    let binary_folder = platform.binary_folder_name(content.model_description.fmi_major_version)?;

    let extension = content.library_path.extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default();

    zip.start_file(
        format!(
            "binaries/{}/{}.{}",
            binary_folder,
            content.model_description.model_identifier,
            extension
        ),
        options,
    )?;
    io::copy(&mut File::open(&content.library_path)?, &mut zip)?;

    zip.add_directory("resources/", options)?;

    if let Some(resources) = &content.resources {
        add_folder(&mut zip, resources, "resources", options)?;
    }

    zip.finish()?;

    Ok(())
}

/// Recursively adds the content of a folder to the zip file, under the given zip path
fn add_folder(
    zip: &mut ZipWriter<File>,
    folder: &Path,
    zip_path: &str,
    options: SimpleFileOptions
) -> Result<(), CliError> {
    let mut entries: Vec<_> = fs::read_dir(folder)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let entry_zip_path = format!("{}/{}", zip_path, name);

        if entry.file_type()?.is_dir() {
            zip.add_directory(format!("{}/", entry_zip_path), options)?;
            add_folder(zip, &entry.path(), &entry_zip_path, options)?;
        } else {
            zip.start_file(entry_zip_path, options)?;
            io::copy(&mut File::open(entry.path())?, zip)?;
        }
    }

    Ok(())
}
