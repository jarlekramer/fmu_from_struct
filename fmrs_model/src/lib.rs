pub mod fmi_types;
pub mod unimplemented_functions;


pub trait FmrsModel {}

pub mod prelude {
    pub use std::ffi;
    pub use super::fmi_types::fmi3::*;
    pub use super::fmi_types::fmi2::*;
    pub use super::unimplemented_functions::*;
    pub use super::FmrsModel;
    pub use fmrs_model_derive::FmrsModel;
}