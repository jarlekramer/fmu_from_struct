use crate::fmi_types::fmi3::*;
use crate::fmi_types::common::*;

use std::ffi;

// ------------------------------- Model management ------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3InstantiateModelExchange(
    _instanceName: *const ffi::c_char,
    _instantiationToken: *const ffi::c_char,
    _resourcePath: *const ffi::c_char,
    _visible: bool,
    _loggingOn: bool,
    _instanceEnvironment: *mut ffi::c_void,
    _logMessage: fmi3LogMessageCallback
) -> *mut ffi::c_void {
    std::ptr::null_mut()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3InstantiateScheduledExecution(
    _instanceName: *const ffi::c_char,
    _instantiationToken: *const ffi::c_char,
    _resourcePath: *const ffi::c_char,
    _visible: bool,
    _loggingOn: bool,
    _instanceEnvironment: *mut ffi::c_void,
    _logMessage: fmi3LogMessageCallback,
    _clockUpdate: fmi3ClockUpdateCallback,
    _lockPreemption: fmi3LockPreemptionCallback,
    _unlockPreemption: fmi3UnlockPreemptionCallback,
) -> *mut ffi::c_void {
    std::ptr::null_mut()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterEventMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3Terminate(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3Reset(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterConfigurationMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ExitConfigurationMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

// ------------------------------- Co-simulation ---------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterStepMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetOutputDerivatives(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _orders: *const u32,
    _values: *mut f64,
    _nValues: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ActivateModelPartition(
    _instance: *mut ffi::c_void,
    _clockReference: u32,
    _activationTime: f64,
) -> FmiStatus {
    FmiStatus::Error
}

// --------------------------- Model Exchange ------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterContinuousTimeMode(_instance: *mut ffi::c_void) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3CompletedIntegratorStep(
    _instance: *mut ffi::c_void,
    _noSetFMUStatePriorToCurrentPoint: bool,
    _enterEventMode: *mut bool,
    _terminateSimulation: *mut bool,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetTime(
    _instance: *mut ffi::c_void,
    _time: f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetContinuousStates(
    _instance: *mut ffi::c_void,
    _continousStates: *const f64,
    _nContinuousStates: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetContinuousStateDerivatives(
    _instance: *mut ffi::c_void,
    _derivatives: *mut f64,
    _nContinuousStates: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetEventIndicators(
    _instance: *mut ffi::c_void,
    _eventIndicators: *mut f64,
    _nEventIndicators: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetContinuousStates(
    _instance: *mut ffi::c_void,
    _continousStates: *mut f64,
    _nContinuousStates: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNominalsOfContinuousStates(
    _instance: *mut ffi::c_void,
    _nominals: *mut f64,
    _nContinuousStates: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfEventIndicators(
    _instance: *mut ffi::c_void,
    _nEventIndicators: *mut usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfContinuousStates(
    _instance: *mut ffi::c_void,
    _nContinuousStates: *mut usize,
) -> FmiStatus {
    FmiStatus::Error
}

// --------------------------- Discrete states -----------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EvaluateDiscreteStates(
    _instance: *mut ffi::c_void,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3UpdateDiscreteStates(
    _instance: *mut ffi::c_void,
    _discreteStatesNeedUpdate: *mut bool,
    _terminateSimulation: *mut bool,
    _nominalsOfContinuousStatesChanged: *mut bool,
    _valuesOfContinuousStatesChanged: *mut bool,
    _nextEventTimeDefined: *mut bool,
    _nextEventTime: *mut f64,
) -> FmiStatus {
    FmiStatus::Error
}

// -------------------------- Set and get ----------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi3GetBinary(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _values: *mut *const u8,
    _nValues: usize,
    _sizes: *mut usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi3GetClock(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _values: *mut bool,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfVariableDependencies(
    _instance: *mut ffi::c_void,
    _valueReference: u32,
    _nDependencies: *mut usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetVariableDependencies(
    _instance: *mut ffi::c_void,
    _dependent: u32,
    _elementIndicesOfDependent: *mut usize,
    _independents: *mut u32,
    _elementIndicesOfIndependents: *mut usize,
    _dependencyKinds: *mut fmi3DependencyKind,
    _nDependencies: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetDirectionalDerivative(
    _instance: *mut ffi::c_void,
    _unknowns: *const u32,
    _nUnknowns: usize,
    _knowns: *const u32,
    _nKnowns: usize,
    _seed: *const f64,
    _nSeed: usize,
    _sensitivity: *mut f64,
    _nSensitivity: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetAdjointDerivative(
    _instance: *mut ffi::c_void,
    _unknowns: *const u32,
    _nUnknowns: usize,
    _knowns: *const u32,
    _nKnowns: usize,
    _seed: *const f64,
    _nSeed: usize,
    _sensitivity: *mut f64,
    _nSensitivity: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetIntervalDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _interval: *mut f64,
    _qualifiers: *mut fmi3IntervalQualifier
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetIntervalFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _interval: *mut f64,
    _qualifiers: *mut fmi3IntervalQualifier
) -> FmiStatus {
    FmiStatus::Error
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetShiftDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shifts: *mut f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetShiftFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shiftCounters: *mut u64,
    _resolutions: *mut u64
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetDebugLogging(
    _instance: *mut ffi::c_void,
    _loggingOn: bool,
    _nCategories: usize,
    _categories: *const *const ffi::c_char,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetBinary(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _values: *const *const u8,
    _nValues: usize,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetClock(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _values: *const bool,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetIntervalDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _intervals: *const f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetIntervalFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _intervalCounters: *const u64,
    _resolutions: *const u64
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetShiftDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shifts: *const f64,
) -> FmiStatus {
    FmiStatus::Error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetShiftFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shiftCounters: *const u64,
    _resolutions: *const u64
) -> FmiStatus {
    FmiStatus::Error
}