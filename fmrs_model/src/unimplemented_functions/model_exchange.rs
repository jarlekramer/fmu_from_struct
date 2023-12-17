
use fmi_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterContinuousTimeMode(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3CompletedIntegratorStep(
    _instance: fmi3Instance,
    _noSetFMUStatePriorToCurrentPoint: bool,
    _enterEventMode: *mut bool,
    _terminateSimulation: *mut bool,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetTime(
    _instance: fmi3Instance,
    _time: f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetContinuousStates(
    _instance: fmi3Instance,
    _continousStates: *const f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetContinuousStateDerivatives(
    _instance: fmi3Instance,
    _derivatives: *mut f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetEventIndicators(
    _instance: fmi3Instance,
    _eventIndicators: *mut f64,
    _nEventIndicators: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetContinuousStates(
    _instance: fmi3Instance,
    _continousStates: *mut f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNominalsOfContinuousStates(
    _instance: fmi3Instance,
    _nominals: *mut f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfEventIndicators(
    _instance: fmi3Instance,
    _nEventIndicators: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfContinuousStates(
    _instance: fmi3Instance,
    _nContinuousStates: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}