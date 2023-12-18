//! Collection of custom types that is necessary to implement the FMI2 standard. The names follows
//! the original c syntax exactly, and is therefore not idomatic rust. This necessary to allow the 
//! types to be exposed to the c interface.
//! 
//! Note: the fmi types that are directly tranlasted to a standard type, e.g. fmi3Float32, are not
//! defined here, as it deemed unnecessary. 

use std::ffi;

// ------------------------- Custom names for primitive types --------------------------------------
#[allow(non_camel_case_types)]
pub type fmi2String = *const ffi::c_char;

#[allow(non_camel_case_types)]
pub type fmi2Byte = ffi::c_char;


// -------------------------- Custom enums ---------------------------------------------------------
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi2Status {
    fmi2OK,
    fmi2Warning,
    fmi2Discard,
    fmi2Error,
    fmi2Fatal,
    fmi2Pending,    
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi2Type {
    fmi2ModelExchange,
    fmi2CoSimulation,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi2StatusKind {
    fmi2DoStepStatus,
    fmi2PendingStatus,
    fmi2LastSuccessfulTime,
    fmi2Terminated,
}

// ------------------------- Custom function pointers ----------------------------------------------

#[allow(non_camel_case_types)]
pub type fmi2CallbackLogger = extern "C" fn(
    component_environment: *mut ffi::c_void,
    instance_name: fmi2String,
    status: fmi2Status,
    category: fmi2String,
    message: fmi2String,
);

#[allow(non_camel_case_types)]
pub type fmi2CallbackAllocateMemory = extern "C" fn(
    nobj: usize,
    size: usize,
) -> *mut ffi::c_void;

#[allow(non_camel_case_types)]
pub type fmi2CallbackFreeMemory = extern "C" fn(
    obj: *mut ffi::c_void,
);

#[allow(non_camel_case_types)]
pub type fmi2StepFinished = extern "C" fn(
    component_environment: *mut ffi::c_void,
    status: fmi2Status,
);

// ------------------------- Custom structs --------------------------------------------------------
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fmi2CallbackFunctions {
    logger: fmi2CallbackLogger,
    allocate_memory: fmi2CallbackAllocateMemory,
    free_memory: fmi2CallbackFreeMemory,
    step_finished: fmi2StepFinished,
    component_environment: *mut ffi::c_void,
}