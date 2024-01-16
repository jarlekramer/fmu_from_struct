//! Collection of custom types that is necessary to implement the FMI2 standard. The names follows
//! the original c syntax exactly, and is therefore not idomatic rust.
//! 
//! Note: the fmi types that are directly tranlasted to a standard type, e.g. fmi3Float32, are not
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
pub struct fmi2CallbackFunctions {
    logger: fmi2CallbackLogger,
    allocate_memory: fmi2CallbackAllocateMemory,
    free_memory: fmi2CallbackFreeMemory,
    step_finished: fmi2StepFinished,
    component_environment: *mut ffi::c_void,
}