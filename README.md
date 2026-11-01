# fmu_from_struct
A derive macro for automatically setting up FMU models in Rust

## Development Status
**WARNING:** This crate is still in development. Bugs can be expected in the current version and breaking changes are to be expected in future versions. The reason it is released openly is mostly to allow people interested in the purpose of this crate (see below) to contribute to and comment on the development.

Only the most fundamental functionality in the FMI standard is so far implemented. However, this is often enough for simple models and use cases. The library is therefore used to implement FMUs from Rust code in separate projects related to maritime research. One example is an FMU-version of the [Stormbird library](https://github.com/NTNU-IMT/stormbird). The basic functionality seems to work as intended.

## Purpose
Automate the implementation of a functional mockup interface for models written in Rust. That is, automatically generate everything from initialization, setters and getters, model description files and everything else needed for an FMI-model. The only thing you have to write is the do-step function. Custom initialization functionality is also possible, but optional (only required sometimes)

Co-simulation is seen as the primary use case for this macro and is currently the only supported type of implementation. Model exchange may be added in the future if there is a need or interest from users.

Only versions 2 and 3 of the FMI standard are supported, with no plans to add support for version 1. Due to how this code is used in other projects by the developer, version 2 is tested more than version 3, and may be considered the safer choice. However, version 3 is theoretically supported in the same way, just not tested as thoroughly.

## Minimal Example
The crate must be compiled as a dynamic library. In `Cargo.toml`:

```toml
[package]
name = "gain_adder"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
fmu_from_struct = "0.3"
```

In `src/lib.rs`:

```rust
use fmu_from_struct::prelude::*;

#[derive(Fmu, Default)]
#[fmu_from_struct(fmi_version = 2)]
pub struct GainAdder {
    #[fmu_from_struct(parameter)]
    #[fmu_from_struct(start_value = 1.0)]
    pub gain: f64,
    #[fmu_from_struct(input)]
    pub a: f64,
    pub b: f64,
    #[fmu_from_struct(output)]
    pub sum: f64,
}

impl FmuFunctions for GainAdder {
    fn do_step(&mut self, _current_time: f64, _time_step: f64) {
        self.sum = self.gain * (self.a + self.b);
    }
}
```

The most important rules are:
- Every `pub` field becomes an FMI variable. Private fields are ignored by the interface, and can be used for internal state.
- The causality attributes `parameter`, `input` and `output` apply to the field they are placed on *and all following fields*, until a new causality attribute is given. Fields are parameters by default. In the example above, `a` and `b` are both inputs.
- The `start_value` attribute sets the start value of a field, both in the model description and in the model itself. Fields without a start value get the value from `Default`.
- The struct must implement `Default`, which is used to create the model before the parameters are set by the importer.
- The supported field types are `f64`, `i32`, `bool` and `String`. FMI 3 also supports `f32`, `i8`, `i16`, `i64`, `u8`, `u16`, `u32` and `u64`.
- The `FmuFunctions` trait requires `do_step`. It also has an optional `exit_initialization_mode` function, which can be used to set up internal state from the parameters before the simulation starts.

## Building the FMU
Running `cargo build` compiles the shared library and writes the generated model description to `target/fmu/<crate_name>/modelDescription.xml`, where it can be inspected. The same model description is also embedded in the compiled library. The file is only rewritten when the crate is recompiled, so after switching between, e.g., debug and release builds, it may show the description from a different build than the latest one. The embedded copy always matches the library it is in, and is the one that ends up in the FMU.

To build *and* package everything into an FMU file, use the cargo subcommand [cargo-fmu-from-struct](https://github.com/jarlekramer/fmu_from_struct/tree/main/cargo-fmu-from-struct), run from the crate folder:

```
cargo install cargo-fmu-from-struct
cargo fmu-from-struct --release
```

This gives `target/fmu/release/GainAdder.fmu`. All arguments except the ones below are passed on to `cargo build` (e.g., `--release`, `-p`, `--features`, `--target`). Options:
- `--resources <PATH>`: folder to include as the `resources` folder in the FMU. By default, the `resources` folder in the crate is used, if it exists. Otherwise, the FMU gets an empty `resources` folder.
- `-o, --output <PATH>`: path of the generated FMU file.

A plain `cargo build` can not produce the FMU on its own, as cargo has no way to run code after a build is finished.

## Getting Started
Check the [examples](https://github.com/jarlekramer/fmu_from_struct/tree/main/examples) folder for more complete code examples on how to use this macro. In particular, there is an example called "spring" which attempts to show the most important ways to use this macro with a simple spring model, including how to use the optional `FmuInfo` field, how to select the FMI version with cargo features, and how to run the FMU from Python with [FMPy](https://github.com/CATIA-Systems/FMPy).

Another example, from an external github page, is the implementation of the [stormbird lifting line FMU](https://github.com/NTNU-IMT/stormbird/tree/main/interfaces/fmus/stormbird_lifting_line). This example is more complex than the basic spring example, but the same basic steps and architecture are used.

## Folder Structure
This repository contains several related crates. This README is the main documentation for all of them.
- `fmu_from_struct` contains the high-level rust code for the macro (mainly prelude and traits). This is the crate to depend on.
- `fmu_from_struct_derive` contains the actual derive macro logic (must be a separate crate). It is re-exported by `fmu_from_struct`, and should not be used directly.
- `cargo-fmu-from-struct` contains a cargo subcommand that builds a crate using the macro and packages the result into a single zip file that follows the FMI standard.
- `package_fmu_after_build` contains the old packaging tool, which is deprecated and replaced by `cargo-fmu-from-struct`. It only works with `fmu_from_struct` 0.2.x and earlier.
- `examples` contains simple examples of how to use the macro

## Contribute
If you would like to contribute, that's great! Please contact the repository owner for an informal discussion about how and what.

If you only would like to raise an issue, feel free to create one in this repository. However, no promises on when it will be fixed...

## License
This macro is licensed under the MIT License. See [LICENSE](https://github.com/jarlekramer/fmu_from_struct/blob/main/LICENSE) or https://opensource.org/license/MIT for details.
