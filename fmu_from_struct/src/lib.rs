pub mod fmi_types;
pub mod unimplemented_functions;

use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct FmuInfo {
    pub name: String,
    pub resource_path: PathBuf,
}


pub trait FmuFunctions {
    /// Steps the simulation forward in time. Necessary for all models.
    fn do_step(&mut self, current_time: f64, time_step: f64);

    /// Can be used to set internal (private) variables based on parameters from the model
    /// description file.
    fn exit_initialization_mode(&mut self) {
        // Do nothing by default
    }
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
