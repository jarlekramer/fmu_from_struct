use crate::fmi_types::fmi2::*;
use crate::fmi_types::common::*;

use std::ffi;

// ------------------------------- Version ---------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetVersion() -> *const ffi::c_char {
    "2.0".as_ptr() as *const ffi::c_char
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetTypesPlatform() -> *const ffi::c_char {
    "default".as_ptr() as *const ffi::c_char
}

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
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2CancelStep(
    _instance: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Ok
}

// ------------------------------- Serialize -------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SerializedFMUstateSize(
    _instance: *mut ffi::c_void,
    _fmu_state: *mut ffi::c_void,
    _size: *mut usize,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SerializeFMUstate(
    _instance: *mut ffi::c_void,
    _fmu_state: *mut ffi::c_void,
    _serialized_state: *mut ffi::c_char,
    _size: usize,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2DeSerializeFMUstate(
    _instance: *mut ffi::c_void,
    _fmu_state: *mut ffi::c_void,
    _serialized_state: *const ffi::c_char,
    _size: usize,
) -> FmiStatus {
    FmiStatus::Ok
}
// ------------------------------- Getting and setting variables -----------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetFMUstate(
    _instance: *mut ffi::c_void,
    _fmu_state: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetFMUstate(
    _instance: *mut ffi::c_void,
    _fmu_state: *const ffi::c_void,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2FreeFMUstate(
    _instance: *mut ffi::c_void,
    _fmu_state: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetDebugLogging(
    _instance: *mut ffi::c_void,
    _logging_on: bool,
    _n_categories: usize,
    _categories: *const *const ffi::c_char,
) -> FmiStatus {
    FmiStatus::Ok
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
    FmiStatus::Ok
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
    FmiStatus::Ok
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
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut FmiStatus,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetRealStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut f64,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetIntegerStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut i32,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetBooleanStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut bool,
) -> FmiStatus {
    FmiStatus::Ok
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi2GetStringStatus(
    _instance: *mut ffi::c_void,
    _status_kind: fmi2StatusKind,
    _value: *mut *const ffi::c_char,
) -> FmiStatus {
    FmiStatus::Ok
}