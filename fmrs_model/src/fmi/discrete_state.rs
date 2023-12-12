
use super::custom_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EvaluateDiscreteStates(
    _instance: fmi3Instance,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3UpdateDiscreteStates(
    _instance: fmi3Instance,
    _discreteStatesNeedUpdate: *mut bool,
    _terminateSimulation: *mut bool,
    _nominalsOfContinuousStatesChanged: *mut bool,
    _valuesOfContinuousStatesChanged: *mut bool,
    _nextEventTimeDefined: *mut bool,
    _nextEventTime: *mut f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}
