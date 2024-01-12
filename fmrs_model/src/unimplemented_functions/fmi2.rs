use crate::fmi_types::fmi2::*;

use std::ffi;

// ------------------------------- Version ---------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetVersion() -> fmi2String {
    "2.0".as_ptr() as fmi2String
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2GetTypesPlatform() -> fmi2String {
    "default".as_ptr() as fmi2String
}

// ------------------------------- Model managment -------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2SetupExperiment(
    _component: *mut ffi::c_void,
    _tolerance_defined: bool,
    _tolerance: f64,
    _start_time: f64,
    _stop_time_defined: bool,
    _stop_time: f64,
) -> fmi2Status {
    fmi2Status::fmi2OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2EnterInitializationMode(
    _component: *mut ffi::c_void,
) -> fmi2Status {
    fmi2Status::fmi2OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2ExitInitializationMode(
    _component: *mut ffi::c_void,
) -> fmi2Status {
    fmi2Status::fmi2OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2Terminate(
    _component: *mut ffi::c_void,
) -> fmi2Status {
    fmi2Status::fmi2OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi2Reset(
    _component: *mut ffi::c_void,
) -> fmi2Status {
    fmi2Status::fmi2OK
}