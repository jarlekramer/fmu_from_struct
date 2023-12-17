use fmi_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SerializedFMUStateSize(
    _instance: fmi3Instance,
    _FMUState: fmi3FMUState,
    _size: *mut usize
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SerializeFMUState(
    _instance: fmi3Instance,
    _FMUState: fmi3FMUState,
    _serializedState: *mut fmi3Byte,
    _size: usize
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3DeserializeFMUState(
    _instance: fmi3Instance,
    _serializedState: *const fmi3Byte,
    _size: usize,
    _FMUState: *mut fmi3FMUState
) -> fmi3Status {
    fmi3Status::fmi3OK
}