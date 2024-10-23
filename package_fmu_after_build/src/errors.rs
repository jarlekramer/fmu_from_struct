
use std::fmt::{self, Display, Formatter};
use std::io;

use zip::result::ZipError;
use xmltree::ParseError;

#[derive(Debug)]
pub enum PackageFmuError {
    NoFolderPathFromEnv,
    FailedToConvertFolderNameToString,
    FileNotFound,
    FailedToParseModelDescription,
    NoFmiVersionInModelDescription,
    NoModelNameInModelDescription,
    InvalidFmiVersion,
    InvalidOs(String),
    NoBuilderPath,
    ZipError,
    Other(String),
}

impl From<ParseError> for PackageFmuError {
    fn from(_: ParseError) -> Self {
        PackageFmuError::FailedToParseModelDescription
    }
}

impl From<ZipError> for PackageFmuError {
    fn from(_: ZipError) -> Self {
        PackageFmuError::ZipError
    }
}

impl From<io::Error> for PackageFmuError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => PackageFmuError::FileNotFound,
            _ => PackageFmuError::Other(error.to_string()),
        }
    }
}

impl Display for PackageFmuError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PackageFmuError::NoFolderPathFromEnv => write!(
                f, "Failed to get current folder name from the environment"
            ),
            PackageFmuError::FailedToConvertFolderNameToString => write!(
                f, "Failed to convert folder name to string"
            ),
            PackageFmuError::FileNotFound => write!(
                f, "modelDescription.xml not found. Are you in the right folder? Have you executed 'cargo build'?"
            ),
            PackageFmuError::FailedToParseModelDescription => write!(
                f, "Failed to parse modelDescription.xml"
            ),
            PackageFmuError::NoFmiVersionInModelDescription => write!(
                f, "fmiVersion not found in modelDescription.xml"
            ),
            PackageFmuError::NoModelNameInModelDescription => write!(
                f, "modelName not found in modelDescription.xml"
            ),
            PackageFmuError::InvalidFmiVersion => write!(
                f, "Invalid fmiVersion"
            ),
            PackageFmuError::InvalidOs(os_string) => write!(
                f, "Invalid OS: {}", os_string
            ),
            PackageFmuError::NoBuilderPath => write!(
                f, "The build folder does not exist. Have you executed 'cargo build' and specified the correct build type for this script (release or not)?"
            ),
            PackageFmuError::ZipError => write!(
                f, "Error from the zip library"
            ),
            PackageFmuError::Other(message) => write!(
                f, "{}", message
            ),

        }
    }
}