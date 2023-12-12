use super::custom_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterStepMode(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetOutputDerivatives(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _orders: *const u32,
    _values: *mut f64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3DoStep(
    _instance: fmi3Instance,
    _currentCommunicationPoint: f64,
    _communicationStepSize: f64,
    _noSetFMUStatePriorToCurrentPoint: bool,
    _eventHandlingNeeded: *mut bool,
    _terminateSimulation: *mut bool,
    _earlyReturn: *mut bool,
    _lastSuccessfulTime: *mut f64,
) -> fmi3Status {
    
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ActivateModelPartition(
    _instance: fmi3Instance,
    _clockReference: fmi3ValueReference,
    _activationTime: f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}