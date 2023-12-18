use crate::fmi_types::fmi3::*;

use libc;
use std::ffi;

// ------------------------------- Version ---------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetVersion() -> fmi3String {
    "3.0".as_ptr() as fmi3String
}

// ------------------------------- Model management ------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3InstantiateModelExchange(
    _instanceName: fmi3String,
    _instantiationToken: fmi3String,
    _resourcePath: fmi3String,
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
    _instanceName: fmi3String,
    _instantiationToken: fmi3String,
    _resourcePath: fmi3String,
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
pub extern "C" fn fmi3EnterInitializationMode(
    _instance: *mut ffi::c_void,
    _toleranceDefined: bool,
    _tolerance: f64,
    _startTime: f64,
    _stopTimeDefined: bool,
    _stopTime: f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ExitInitializationMode(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterEventMode(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3Terminate(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3Reset(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterConfigurationMode(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ExitConfigurationMode(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3FreeFMUState(
    _instance: *mut ffi::c_void,
    FMUstate: *mut *mut ffi::c_void,
) -> fmi3Status {
    if FMUstate == std::ptr::null_mut() {
        return fmi3Status::fmi3Error;
    }

    unsafe {
        libc::free(FMUstate as *mut libc::c_void);

        *FMUstate = std::ptr::null_mut();
    }

    fmi3Status::fmi3OK
}


// ------------------------------- Co-simulation ---------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterStepMode(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
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
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ActivateModelPartition(
    _instance: *mut ffi::c_void,
    _clockReference: u32,
    _activationTime: f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

// --------------------------- Model Exchange ------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterContinuousTimeMode(_instance: *mut ffi::c_void) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3CompletedIntegratorStep(
    _instance: *mut ffi::c_void,
    _noSetFMUStatePriorToCurrentPoint: bool,
    _enterEventMode: *mut bool,
    _terminateSimulation: *mut bool,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetTime(
    _instance: *mut ffi::c_void,
    _time: f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetContinuousStates(
    _instance: *mut ffi::c_void,
    _continousStates: *const f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetContinuousStateDerivatives(
    _instance: *mut ffi::c_void,
    _derivatives: *mut f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetEventIndicators(
    _instance: *mut ffi::c_void,
    _eventIndicators: *mut f64,
    _nEventIndicators: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetContinuousStates(
    _instance: *mut ffi::c_void,
    _continousStates: *mut f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNominalsOfContinuousStates(
    _instance: *mut ffi::c_void,
    _nominals: *mut f64,
    _nContinuousStates: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfEventIndicators(
    _instance: *mut ffi::c_void,
    _nEventIndicators: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfContinuousStates(
    _instance: *mut ffi::c_void,
    _nContinuousStates: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

// --------------------------- Serialize -----------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SerializedFMUStateSize(
    _instance: *mut ffi::c_void,
    _FMUState: *mut ffi::c_void,
    _size: *mut usize
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SerializeFMUState(
    _instance: *mut ffi::c_void,
    _FMUState: *mut ffi::c_void,
    _serializedState: *mut fmi3Byte,
    _size: usize
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3DeserializeFMUState(
    _instance: *mut ffi::c_void,
    _serializedState: *const fmi3Byte,
    _size: usize,
    _FMUState: *mut *mut ffi::c_void
) -> fmi3Status {
    fmi3Status::fmi3OK
}

// --------------------------- Discrete states -----------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EvaluateDiscreteStates(
    _instance: *mut ffi::c_void,
) -> fmi3Status {
    fmi3Status::fmi3OK
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
) -> fmi3Status {
    fmi3Status::fmi3OK
}

// -------------------------- Set and get ----------------------------------------------------------
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetString(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _values: *mut fmi3String,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi3GetBinary(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _values: *mut fmi3Binary,
    _nValues: usize,
    _sizes: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi3GetClock(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _values: *mut bool,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfVariableDependencies(
    _instance: *mut ffi::c_void,
    _valueReference: u32,
    _nDependencies: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
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
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetFMUState(
    _instance: *mut ffi::c_void,
    _FMUState: *mut *mut ffi::c_void,
) -> fmi3Status {
    fmi3Status::fmi3OK
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
) -> fmi3Status {
    fmi3Status::fmi3OK
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
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetIntervalDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _interval: *mut f64,
    _qualifiers: *mut fmi3IntervalQualifier
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetIntervalFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _interval: *mut f64,
    _qualifiers: *mut fmi3IntervalQualifier
) -> fmi3Status {
    fmi3Status::fmi3OK
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetShiftDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shifts: *mut f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetShiftFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shiftCounters: *mut u64,
    _resolutions: *mut u64
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetDebugLogging(
    _instance: *mut ffi::c_void,
    _loggingOn: bool,
    _nCategories: usize,
    _categories: *const fmi3String,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetString(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _values: *const fmi3String,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetBinary(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _values: *const fmi3Binary,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetClock(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _values: *const bool,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetFMUState(
    _instance: *mut ffi::c_void,
    _FMUState: *mut ffi::c_void
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetIntervalDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _intervals: *const f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetIntervalFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _intervalCounters: *const u64,
    _resolutions: *const u64
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetShiftDecimal(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shifts: *const f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetShiftFraction(
    _instance: *mut ffi::c_void,
    _valueReferences: *const u32,
    _nValueReferences: usize,
    _shiftCounters: *const u64,
    _resolutions: *const u64
) -> fmi3Status {
    fmi3Status::fmi3OK
}