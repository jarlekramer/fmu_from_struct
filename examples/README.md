## Build Instructions
To build any of the examples do the following.

### Development build:
```
cd <example folder>
cargo build
python -m fmu_build_utils.package_fmu
```

### Release build:
```
cd <example folder>
cargo build --release
python -m fmu_build_utils.package_fmu --release
```