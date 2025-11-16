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
}
