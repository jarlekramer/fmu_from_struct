
//! Rust implementation of the functions in the FMI3 standard that assumes the custom model in this
//! crate is used to handle the model logic.

pub mod co_simulation;
pub mod model_exchange;
pub mod setters;
pub mod getters;
pub mod model_managment;
pub mod serialize;
pub mod discrete_state;

use fmi_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetVersion() -> fmi3String {
    "3.0".as_ptr() as fmi3String
}