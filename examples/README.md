## Build Instructions
To build any of the examples do the following.

### Development build:
```
cd <example folder>
cargo build
```

### Release build:
```
cd <example folder>
cargo build --release
```

### Automated packaging
If the command line tool `package_fmu_after_build` is installed, you can package the FMU after building it by running:
```
package_fmu_after_build
```

or, for a release build:
```
package_fmu_after_build --release
```
