use crate::fmi_types::fmi2::*;
use crate::fmi_types::common::*;

use std::ffi;

// ------------------------------- Model management ------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetupExperiment(
    _instance: *mut ffi::c_void,
    _tolerance_defined: i32,
    _tolerance: f64,
    _start_time: f64,
    _stop_time_defined: i32,
    _stop_time: f64,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2Terminate(
    _instance: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2CancelStep(
    _instance: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Error
}

// ------------------------------- Getting and setting variables -----------------------------------
/// Logging is not supported yet, so the request is ignored. Returns OK, as importers may call this
/// function whenever logging is enabled, and fail if it returns an error.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetDebugLogging(
    _instance: *mut ffi::c_void,
    _logging_on: i32,
    _n_categories: usize,
    _categories: *const *const ffi::c_char,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetDirectionalDerivative(
    _instance: *mut ffi::c_void,
    _unknown_value_references: *const u32,
    _n_unknowns: usize,
    _known_value_references: *const u32,
    _n_knowns: usize,
    _delta_knowns: *const f64,
    _delta_unknowns: *mut f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetRealInputDerivatives(
    _instance: *mut ffi::c_void,
    _vr: *const u32,
    _nvr: usize,
    _order: *const i32,
    _value: *const f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetRealOutputDerivatives(
    _instance: *mut ffi::c_void,
    _vr: *const u32,
    _nvr: usize,
    _order: *const i32,
    _value: *mut f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut FmiStatus,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetRealStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetIntegerStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut i32,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetBooleanStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut i32,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi2GetStringStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut *const ffi::c_char,
) -> FmiStatus {
    FmiStatus::Error
}

// --------------------------- Model Exchange ------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2EnterEventMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2NewDiscreteStates(
    _instance: *mut ffi::c_void,
    _event_info: *mut fmi2EventInfo,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2EnterContinuousTimeMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2CompletedIntegratorStep(
    _instance: *mut ffi::c_void,
    _no_set_fmu_state_prior_to_current_point: i32,
    _enter_event_mode: *mut i32,
    _terminate_simulation: *mut i32,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetTime(
    _instance: *mut ffi::c_void,
    _time: f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetContinuousStates(
    _instance: *mut ffi::c_void,
    _states: *const f64,
    _n_states: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetDerivatives(
    _instance: *mut ffi::c_void,
    _derivatives: *mut f64,
    _n_states: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetEventIndicators(
    _instance: *mut ffi::c_void,
    _event_indicators: *mut f64,
    _n_event_indicators: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetContinuousStates(
    _instance: *mut ffi::c_void,
    _states: *mut f64,
    _n_states: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetNominalsOfContinuousStates(
    _instance: *mut ffi::c_void,
    _nominals: *mut f64,
    _n_states: usize,
) -> FmiStatus {
    FmiStatus::Error
}
