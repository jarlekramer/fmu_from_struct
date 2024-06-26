//! Collection of custom types that is necessary to implement the FMI3 standard. The names follows
//! the original c syntax exactly, and is therefore not idomatic rust.
//! 
//! Note: the fmi types that are directly tranlasted to a standard type, e.g. fmi3Float32, are not
//! defined here, as it deemed unnecessary. 

use std::ffi;

use crate::fmi_types::common::*;

// ------------------------- Custom enums ----------------------------------------------------------
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi3DependencyKind {
    fmi3Independent,
    fmi3Constant,
    fmi3Fixed,
    fmi3Tunable,
    fmi3Discrete,
    fmi3Dependent,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi3IntervalQualifier {
    fmi3IntervalNotYetKnown,
    fmi3IntervalUnchanged,
    fmi3IntervalChanged
}

// ------------------- Custom function pointers ----------------------------------------------------
#[allow(non_camel_case_types)]
pub type fmi3LogMessageCallback = extern "C" fn(
    instance_environment: *mut ffi::c_void,
    status: FmiStatus,
    category: *const ffi::c_char,
    message: *const ffi::c_char,
);

#[allow(non_camel_case_types)]
pub type fmi3ClockUpdateCallback = extern "C" fn(
    instance_environment: *mut ffi::c_void,
);

#[allow(non_camel_case_types)]
pub type fmi3IntermediateUpdateCallback = extern "C" fn(
    instance_environment: *mut ffi::c_void,
    intermediate_update_time: f64,
    intermediate_variable_set_requested: bool,
    intermediate_variable_get_allowed: bool,
    intermediate_step_finished: bool,
    can_return_early: bool,
    early_return_requested: *mut bool,
    early_return_time: *mut f64,
);

#[allow(non_camel_case_types)]
pub type fmi3LockPreemptionCallback = extern "C" fn();

#[allow(non_camel_case_types)]
pub type fmi3UnlockPreemptionCallback = extern "C" fn();