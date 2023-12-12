use super::custom_types::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetFloat32(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut f32,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetFloat64(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut f64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetInt8(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut i8,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetUInt8(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut u8,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetInt16(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut i16,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetUInt16(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut u16,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetInt32(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut i32,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetUInt32(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut u32,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetInt64(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut i64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetUInt64(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut u64,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetBoolean(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut bool,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetString(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _values: *mut fmi3String,
    _nValues: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fmi3GetBinary(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
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
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _values: *mut bool,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetNumberOfVariableDependencies(
    _instance: fmi3Instance,
    _valueReference: fmi3ValueReference,
    _nDependencies: *mut usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetVariableDependencies(
    _instance: fmi3Instance,
    _dependent: fmi3ValueReference,
    _elementIndicesOfDependent: *mut usize,
    _independents: *mut fmi3ValueReference,
    _elementIndicesOfIndependents: *mut usize,
    _dependencyKinds: *mut fmi3DependencyKind,
    _nDependencies: usize,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetFMUState(
    _instance: fmi3Instance,
    _FMUState: *mut fmi3FMUState,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetDirectionalDerivative(
    _instance: fmi3Instance,
    _unknowns: *const fmi3ValueReference,
    _nUnknowns: usize,
    _knowns: *const fmi3ValueReference,
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
    _instance: fmi3Instance,
    _unknowns: *const fmi3ValueReference,
    _nUnknowns: usize,
    _knowns: *const fmi3ValueReference,
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
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _interval: *mut f64,
    _qualifiers: *mut fmi3IntervalQualifier
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetIntervalFraction(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _interval: *mut f64,
    _qualifiers: *mut fmi3IntervalQualifier
) -> fmi3Status {
    fmi3Status::fmi3OK
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetShiftDecimal(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _shifts: *mut f64,
) -> fmi3Status {
    fmi3Status::fmi3OK
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn fmi3GetShiftFraction(
    _instance: fmi3Instance,
    _valueReferences: *const fmi3ValueReference,
    _nValueReferences: usize,
    _shiftCounters: *mut u64,
    _resolutions: *mut u64
) -> fmi3Status {
    fmi3Status::fmi3OK
}