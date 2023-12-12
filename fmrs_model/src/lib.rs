pub mod fmi;

pub trait FmrsModel {}

pub mod prelude {
    pub use super::fmi::*;
    pub use super::FmrsModel;
    pub use fmrs_model_derive::FmrsModel;
}