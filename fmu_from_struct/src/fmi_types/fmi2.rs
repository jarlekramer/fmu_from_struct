//! Collection of custom types that is necessary to implement the FMI2 standard. The names follows
//! the original c syntax exactly, and is therefore not idiomatic rust.
//! 
//! Note: the fmi types that are directly translated to a standard type, e.g. fmi3Float32, are not
//! defined here, as it deemed unnecessary. 

use std::ffi;

use crate::fmi_types::common::*;


// -------------------------- Custom enums ---------------------------------------------------------
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi2Type {
    fmi2ModelExchange,
    fmi2CoSimulation,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi2StatusKind {
    fmi2DoStepStatus,
    fmi2PendingStatus,
    fmi2LastSuccessfulTime,
    fmi2Terminated,
}

// ------------------------- Custom function pointers ----------------------------------------------

#[allow(non_camel_case_types)]
pub type fmi2CallbackLogger = extern "C" fn(
    component_environment: *mut ffi::c_void,
    instance_name: *const ffi::c_char,
    status: FmiStatus,
    category: *const ffi::c_char,
    message: *const ffi::c_char,
);

#[allow(non_camel_case_types)]
pub type fmi2CallbackAllocateMemory = extern "C" fn(
    nobj: usize,
    size: usize,
) -> *mut ffi::c_void;

#[allow(non_camel_case_types)]
pub type fmi2CallbackFreeMemory = extern "C" fn(
    obj: *mut ffi::c_void,
);

#[allow(non_camel_case_types)]
pub type fmi2StepFinished = extern "C" fn(
    component_environment: *mut ffi::c_void,
    status: FmiStatus,
);

// ------------------------- Custom structs --------------------------------------------------------
#[repr(C)]
#[allow(non_camel_case_types)]
/// The callback functions passed to `fmi2Instantiate`. The function pointers are wrapped in an
/// `Option`, as the importer is allowed to pass NULL for some of them (e.g., `step_finished`).
pub struct fmi2CallbackFunctions {
    pub logger: Option<fmi2CallbackLogger>,
    pub allocate_memory: Option<fmi2CallbackAllocateMemory>,
    pub free_memory: Option<fmi2CallbackFreeMemory>,
    pub step_finished: Option<fmi2StepFinished>,
    pub component_environment: *mut ffi::c_void,
}

#[repr(C)]
#[allow(non_camel_case_types)]
/// Event information returned by `fmi2NewDiscreteStates` (model exchange). The booleans are
/// `fmi2Boolean`, i.e., C `int`.
pub struct fmi2EventInfo {
    pub new_discrete_states_needed: i32,
    pub terminate_simulation: i32,
    pub nominals_of_continuous_states_changed: i32,
    pub values_of_continuous_states_changed: i32,
    pub next_event_time_defined: i32,
    pub next_event_time: f64,
}