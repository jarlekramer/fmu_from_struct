# cargo-fmu-from-struct

A cargo subcommand that builds a crate that uses the [fmu_from_struct](https://crates.io/crates/fmu_from_struct) macro and packages the result as an FMU.

See the [main README](https://github.com/jarlekramer/fmu_from_struct) for the full documentation and a minimal example.

## Installation
```
cargo install cargo-fmu-from-struct
```

## Usage
Run the subcommand from the crate (or workspace) folder. All arguments that are not listed below are passed on to `cargo build`:

```
cargo fmu-from-struct                         # debug build
cargo fmu-from-struct --release               # release build
cargo fmu-from-struct --release -p my_fmu     # select a crate in a workspace
```

The FMU is written to `<target dir>/fmu/<profile>/<ModelName>.fmu`, e.g., `target/fmu/release/MyModel.fmu`.

### Options
- `--resources <PATH>`: folder to include as the `resources` folder in the FMU. If not given, the `resources` folder in the crate is used, if it exists. Otherwise, the FMU gets an empty `resources` folder, so that resources can be added later.
- `-o, --output <PATH>`: path of the generated FMU file. Useful for building several FMUs from the same code, but with different resources:
  ```
  cargo fmu-from-struct --release --resources config_a -o FmuA.fmu
  cargo fmu-from-struct --release --resources config_b -o FmuB.fmu
  ```

### Platforms
The FMU contains the binary for the platform that is built for, which is the current platform unless `--target` is used. Windows, Linux and macOS are supported, on x86, x86_64 and aarch64.
