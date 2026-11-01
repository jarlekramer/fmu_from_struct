## Build Instructions
To build any of the examples, install the cargo subcommand `cargo-fmu-from-struct` (once):
```
cargo install cargo-fmu-from-struct
```

or, when working on this repository, install the local version:
```
cargo install --path ../cargo-fmu-from-struct
```

Then build and package the FMU:
```
cd <example folder>
cargo fmu-from-struct
```

or, for a release build:
```
cargo fmu-from-struct --release
```

The FMU is written to `target/fmu/debug/<ModelName>.fmu` or `target/fmu/release/<ModelName>.fmu`.
