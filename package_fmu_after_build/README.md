# Package FMU After Build

This command line tool is intended to be used together with the derive macro [fmu_from_struct](https://github.com/jarlekramer/fmu_from_struct)

Its job is to package files after a build into a zip file that supports the FMU-standard.

See the [main README](https://github.com/jarlekramer/fmu_from_struct) for the full documentation and a minimal example.

## Usage
Run the tool from the folder of the crate that uses the macro, after building it. The crate folder must have the same name as the crate. It will only make a FMU that supports the operating system that is currently running (should be improved later...)

For a debug build:

```
package_fmu_after_build
```

For a release build:

```
package_fmu_after_build --release
```
