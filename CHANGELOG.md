# Change log fmu_from_struct

## 0.3.0 - Unreleased

### Breaking changes

### New features
- The `start_value` attribute now also accepts plain literals, e.g., `fmu_from_struct(start_value = 1.0)`, `fmu_from_struct(start_value = -2)` or `fmu_from_struct(start_value = true)`, in addition to strings, e.g., `fmu_from_struct(start_value = "1.0")`. The 0.2.1 entry below already showed the literal form, but it was not supported until now.
- Panics in user code (`do_step`, `exit_initialization_mode`, `Default`) are now caught at the FFI boundary and turned into `FmiStatus::Error` (or a NULL instance from `Instantiate`) instead of aborting the importing process.

### Bug fixes
- FMI 2: `fmi2Instantiate` took `fmi2CallbackFunctions` by value instead of as a pointer, as the standard requires. This worked on Windows x64 by accident, but would corrupt the following arguments on, e.g., Linux x86-64.
- FMI 2: `fmi2Boolean` arguments (`visible`, `loggingOn`, `noSetFMUStatePriorToCurrentPoint`, and arguments in the unimplemented stub functions) were declared as Rust `bool` instead of `i32` (C `int`).
- FMI 2: The function pointers in `fmi2CallbackFunctions` are now `Option`s, as the importer is allowed to pass NULL for some of them (e.g., `stepFinished`).
- FMI 3: `fmi3GetBoolean`/`fmi3SetBoolean` used `i32` for the values, while `fmi3Boolean` is a C `bool` (one byte).
- FMI 3: The `logMessage` and `intermediateUpdate` callbacks in `fmi3InstantiateCoSimulation` are now `Option`s, since a NULL function pointer is undefined behaviour in Rust.
- FMI 3: String start values are now written as a `<Start value="..."/>` child element in the model description, as the standard requires, rather than as a `start` attribute.
- FMI 2: Integer (`i32`) variables were written with a `<Real>` element in the model description instead of `<Integer>`.
- Default start values were only defined for `f64`, `bool`, `i32` and `String`, so FMI 3 types such as `f32`, `i8` or `u64` without an explicit `start_value` failed at compile time. All supported types now have a default start value.
- Start values for `String` fields generated invalid code, as the string was inserted as Rust tokens rather than as a string literal.
- Start values are now XML-escaped when written to the model description.
- The resource path in `FmuInfo` was parsed wrongly on Linux: the FMI 2 URI `file:///home/...` became `home/...` and all `/` were replaced by `\`. The URI is now parsed properly (including `file://localhost/...`, `file:/...` and percent-encoded characters like `%20`). Backslashes are only used on Windows. For FMI 3, the resource path is a plain path and is used as is. A NULL resource path gives an empty path rather than undefined behaviour.
- An unknown value reference in a getter or setter now returns `FmiStatus::Error` instead of panicking across the FFI boundary.
- The string getter leaked the memory of every returned string. The strings are now owned by the FMU instance and freed on the next call to the string getter, or when the instance is freed. To make this possible, the instance pointer now points to an internal wrapper, `FmuInstance`, around the user struct. This does not change anything for users of the macro.
- The string getter returns `FmiStatus::Error` for strings with interior NULL bytes instead of panicking, and the string setter returns `FmiStatus::Error` for NULL pointers.
- `fmu_from_struct` depended on the `fmu_from_struct_derive` version on crates.io, even inside this repository. Local changes to the derive macro were therefore not used when building the examples. It now uses a path dependency, with the version kept for publishing.

## 0.2.1 - 2025.11.16
- Reorganized the attributes in the derive macro to be in a "fmu_from_struct" group. Every attribute now has be wrapped using this group. As an example, the attribute `parameter` has been changed to `fmu_from_struct(parameter)`. The point of this is to avoid potential naming conflicts with other macros.
- Added the possibility of using start values for the FMI variables. This can be set by using the following attribute over a field (in this case, setting the start value for a float): `fmu_from_struct(start_value = 42.0)`. The start value needs to be set directly in the attribute text, rather than specifying a function like it is done in, e.g., the serde crate. This is not an ideal solution, but it was chosen for now as the start value is needed when writing the model description file. As far as I can see right now, using functions to set the start value would require custom build scripts for every struct that is turned into an FMU. Need to think about this further in the future...  