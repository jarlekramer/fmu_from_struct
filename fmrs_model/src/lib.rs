pub mod unimplemented_functions;

pub trait FmrsModel {}

pub mod prelude {
    pub use std::ffi;
    pub use fmi_types::*;
    pub use super::unimplemented_functions::*;
    pub use super::FmrsModel;
    pub use fmrs_model_derive::FmrsModel;
}