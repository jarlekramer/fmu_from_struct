pub mod fmi_types;
pub mod unimplemented_functions;

use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
/// Structure to store information about the FMU that will eventually be passed by some of the 
/// standard FMI functions. This structure can, optionally, be added to an struct that might need
/// the information to work properly. The main intended use case is to be able to load resources 
/// based on a path that is dependent on the location of the FMU
pub struct FmuInfo {
    /// The name of the FMU instance
    pub name: String,
    /// The path to the local resource folder
    pub resource_path: PathBuf,
}

#[doc(hidden)]
/// Wrapper around the user struct that is used as the actual FMU instance behind the instance
/// pointer passed to the importer. It makes it possible to store data that the FMI standard needs,
/// without requiring extra fields in the user struct. Only intended to be used by the derive macro.
pub struct FmuInstance<T> {
    /// The user struct that the macro is derived for
    pub model: T,
    /// Buffer for the strings returned by the string getter. The FMI standard requires that the
    /// returned strings are valid until a later function call on the same instance, so the memory
    /// is owned here and freed on the next call to the string getter (or when the instance is
    /// freed).
    pub string_buffer: Vec<std::ffi::CString>,
}

impl<T> FmuInstance<T> {
    pub fn new(model: T) -> Self {
        Self {
            model,
            string_buffer: Vec::new(),
        }
    }
}

#[doc(hidden)]
/// Converts the resource path from the importer to a `PathBuf`. The format of the input depends on
/// the FMI version:
/// - FMI 2: a URI, typically on the form `file:///C:/path/to/resources` or
///   `file:///home/path/to/resources`, possibly with percent-encoded characters.
/// - FMI 3: a plain path.
///
/// A NULL pointer results in an empty path.
pub unsafe fn resource_path_from_c_str(
    resource_path: *const std::ffi::c_char,
    fmi_major_version: u32
) -> PathBuf {
    if resource_path.is_null() {
        return PathBuf::new();
    }

    let resource_path_string = std::ffi::CStr::from_ptr(resource_path)
        .to_string_lossy()
        .into_owned();

    if fmi_major_version == 2 {
        resource_path_from_uri(&resource_path_string)
    } else {
        PathBuf::from(resource_path_string)
    }
}

/// Converts a file URI to a `PathBuf`. Input that is not a file URI is interpreted as a plain path.
fn resource_path_from_uri(uri: &str) -> PathBuf {
    let path = if let Some(rest) = uri.strip_prefix("file://") {
        // Skip the authority, i.e., everything up to the first '/'. Usually empty or 'localhost'
        match rest.find('/') {
            Some(index) => &rest[index..],
            None => rest,
        }
    } else if let Some(rest) = uri.strip_prefix("file:") {
        rest
    } else {
        uri
    };

    let path = percent_decode(path);

    // Windows paths come on the form '/C:/path', where the first slash must be removed
    let bytes = path.as_bytes();
    let is_windows_drive_path = bytes.len() >= 3 &&
        bytes[0] == b'/' &&
        bytes[1].is_ascii_alphabetic() &&
        bytes[2] == b':';

    let path = if is_windows_drive_path {
        path[1..].to_string()
    } else {
        path
    };

    if cfg!(windows) {
        PathBuf::from(path.replace('/', "\\"))
    } else {
        PathBuf::from(path)
    }
}

/// Decodes percent-encoded characters in a URI, e.g., '%20' to ' '
fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut decoded: Vec<u8> = Vec::with_capacity(bytes.len());

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()
                .and_then(|s| u8::from_str_radix(s, 16).ok());

            if let Some(value) = hex {
                decoded.push(value);
                i += 3;
                continue;
            }
        }

        decoded.push(bytes[i]);
        i += 1;
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

/// Functions that must be implemented for any struct that uses this derive macro. The only required
/// one is the `do_step`. The `exit_initialization_mode` has default behavior.
pub trait FmuFunctions {
    /// Steps the simulation forward in time. Necessary for all models.
    fn do_step(&mut self, current_time: f64, time_step: f64);

    /// Can be used to set internal (private) variables based on parameters from the model
    /// description file. It Does nothing by default. Can be overridden if some custom 
    /// initialization behavior is needed  
    fn exit_initialization_mode(&mut self) {}
}

pub mod prelude {
    pub use std::ffi;
    pub use std::ptr;
    pub use std::path::PathBuf;
    pub use super::fmi_types::fmi3::*;
    pub use super::fmi_types::fmi2::*;
    pub use super::fmi_types::common::*;
    pub use super::unimplemented_functions::*;
    pub use super::FmuFunctions;
    pub use fmu_from_struct_derive::Fmu;
    pub use super::FmuInfo;
    pub use super::FmuInstance;
    pub use super::resource_path_from_c_str;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expected(path: &str) -> PathBuf {
        if cfg!(windows) {
            PathBuf::from(path.replace('/', "\\"))
        } else {
            PathBuf::from(path)
        }
    }

    #[test]
    fn windows_uri() {
        assert_eq!(
            resource_path_from_uri("file:///C:/temp/my%20fmu/resources"),
            expected("C:/temp/my fmu/resources")
        );
    }

    #[test]
    fn unix_uri() {
        assert_eq!(
            resource_path_from_uri("file:///home/user/resources"),
            expected("/home/user/resources")
        );
    }

    #[test]
    fn uri_with_localhost_and_short_form() {
        assert_eq!(
            resource_path_from_uri("file://localhost/home/user/resources"),
            expected("/home/user/resources")
        );
        assert_eq!(
            resource_path_from_uri("file:/C:/temp/resources"),
            expected("C:/temp/resources")
        );
    }

    #[test]
    fn invalid_percent_encoding_is_kept() {
        assert_eq!(percent_decode("a%2"), "a%2");
        assert_eq!(percent_decode("a%zz"), "a%zz");
        assert_eq!(percent_decode("a%41"), "aA");
    }
}
