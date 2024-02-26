use crate::fmi_types::fmi2::*;
use crate::fmi_types::common::*;

use std::ffi;

// ------------------------------- Model managment -------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetupExperiment(
    _instance: *mut ffi::c_void,
    _tolerance_defined: bool,
    _tolerance: f64,
    _start_time: f64,
    _stop_time_defined: bool,
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
pub extern "C" fn fmi2Reset(
    _instance: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2CancelStep(
    _instance: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Error
}

// ------------------------------- Getting and setting variables -----------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetDebugLogging(
    _instance: *mut ffi::c_void,
    _logging_on: bool,
    _n_categories: usize,
    _categories: *const *const ffi::c_char,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetDirectionalDerivative(
    _instance: *mut ffi::c_void,
    _v_ref: *const u32,
    _n_v_ref: usize,
    _z_ref: *const u32,
    _n_z_ref: usize,
    _dv: *mut f64,
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
    _value: *mut bool,
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