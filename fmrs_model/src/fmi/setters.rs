use fmi_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetDebugLogging(
    _instance: fmi3Instance,
    _loggingOn: bool,
    _nCategories: usize,
    _categories: *const fmi3String,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetFloat32(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const f32,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetFloat64(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const f64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetInt8(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const i8,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetUInt8(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const u8,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetInt16(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const i16,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetUInt16(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const u16,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetInt32(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const i32,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetUInt32(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const u32,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetInt64(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const i64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetUInt64(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const u64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetBoolean(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const bool,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetString(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const fmi3String,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetBinary(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *const fmi3Binary,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetClock(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _values: *const bool,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetFMUState(
    _instance: fmi3Instance,
    _FMUState: fmi3FMUState
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetIntervalDecimal(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _intervals: *const f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetIntervalFraction(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _intervalCounters: *const u64,
    _resolutions: *const u64
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetShiftDecimal(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _shifts: *const f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3SetShiftFraction(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _shiftCounters: *const u64,
    _resolutions: *const u64
) -> fmi3Status {
    fmi3Status::fmi3OK
}