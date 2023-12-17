use fmi_types::*;

use libc;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3InstantiateModelExchange(
    _instanceName: fmi3String,
    _instantiationToken: fmi3String,
    _resourcePath: fmi3String,
    _visible: bool,
    _loggingOn: bool,
    _instanceEnvironment: fmi3InstanceEnvironment,
    _logMessage: fmi3LogMessageCallback
) -> fmi3Instance {
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
    _instanceEnvironment: fmi3InstanceEnvironment,
    _logMessage: fmi3LogMessageCallback,
    _clockUpdate: fmi3ClockUpdateCallback,
    _lockPreemption: fmi3LockPreemptionCallback,
    _unlockPreemption: fmi3UnlockPreemptionCallback,
) -> fmi3Instance {
    std::ptr::null_mut()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterInitializationMode(
    _instance: fmi3Instance,
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
pub extern "C" fn fmi3ExitInitializationMode(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterEventMode(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3Terminate(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3Reset(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3EnterConfigurationMode(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3ExitConfigurationMode(_instance: fmi3Instance) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3FreeFMUState(
    _instance: fmi3Instance,
    FMUstate: *mut fmi3FMUState,
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
