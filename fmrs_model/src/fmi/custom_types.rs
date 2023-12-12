//! Collection of custom types that is necessary to implement the FMI3 standard. The names follows
//! the original c syntax exactly, and is therefore not idomatic rust. This necessary to allow the 
//! types to be exposed to the c interface.
//! 
//! Note: the fmi types that are directly tranlasted to a standard type, e.g. fmi3Float32, are not
//! defined here, as it deemed unnecessary. 
//! 
//! However, some types where the underlying priomitive type is uncelar, such as fmi3ValueReference, 
//! are defined to increase readability.

use std::ffi;

// -------------------------- Custom names for void pointers ---------------------------------------
#[allow(non_camel_case_types)]
pub type fmi3Instance = *mut ffi::c_void;

#[allow(non_camel_case_types)]
pub type fmi3InstanceEnvironment = *mut ffi::c_void;

#[allow(non_camel_case_types)]
pub type fmi3FMUState = *mut ffi::c_void;

// ------------------------- Custom names for primitive types --------------------------------------
#[allow(non_camel_case_types)]
pub type fmi3ValueReference = u32;

#[allow(non_camel_case_types)]
pub type fmi3String = *const ffi::c_char;

#[allow(non_camel_case_types)]
pub type fmi3Byte = u8;
#[allow(non_camel_case_types)]
pub type fmi3Binary = *const fmi3Byte;


// ------------------------- Custom enums ----------------------------------------------------------
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi3Status {
    fmi3OK,
    fmi3Warning,
    fmi3Discard,
    fmi3Error,
    fmi3Fatal,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi3DependencyKind {
    fmi3Independent,
    fmi3Constant,
    fmi3Fixed,
    fmi3Tunable,
    fmi3Discrete,
    fmi3Dependent,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fmi3IntervalQualifier {
    fmi3IntervalNotYetKnown,
    fmi3IntervalUnchanged,
    fmi3IntervalChanged
}

// ------------------- Custom function pointers ----------------------------------------------------
#[allow(non_camel_case_types)]
pub type fmi3LogMessageCallback = extern "C" fn(
    instanceEnvironment: fmi3InstanceEnvironment,
    status: fmi3Status,
    category: fmi3String,
    message: fmi3String,
);

#[allow(non_camel_case_types)]
pub type fmi3ClockUpdateCallback = extern "C" fn(
    instanceEnvironment: fmi3InstanceEnvironment,
);

#[allow(non_camel_case_types)]
pub type fmi3IntermediateUpdateCallback = extern "C" fn(
    instanceEnvironment: fmi3InstanceEnvironment,
    intermediateUpdateTime: f64,
    intermediateVariableSetRequested: bool,
    intermediateVariableGetAllowed: bool,
    intermediateStepFinished: bool,
    canReturnEarly: bool,
    earlyReturnRequested: *mut bool,
    earlyReturnTime: *mut f64,
);

#[allow(non_camel_case_types)]
pub type fmi3LockPreemptionCallback = extern "C" fn();

#[allow(non_camel_case_types)]
pub type fmi3UnlockPreemptionCallback = extern "C" fn();