use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
/// Error type for the command line tool. All errors are only reported to the user, so a message is
/// all that is needed.
pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        Self::new(error.to_string())
    }
}

impl From<cargo_metadata::Error> for CliError {
    fn from(error: cargo_metadata::Error) -> Self {
        Self::new(format!("Failed to run 'cargo metadata': {}", error))
    }
}

impl From<zip::result::ZipError> for CliError {
    fn from(error: zip::result::ZipError) -> Self {
        Self::new(format!("Failed to write the FMU file: {}", error))
    }
}

impl From<xmltree::ParseError> for CliError {
    fn from(error: xmltree::ParseError) -> Self {
        Self::new(format!("Failed to parse modelDescription.xml: {}", error))
    }
}
