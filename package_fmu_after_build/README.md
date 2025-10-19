# Package FMU After Build

This command line tool is intended to be used together with the derive macro [fmu_from_struct](https://github.com/jarlekramer/fmu_from_struct)

Its job is to package files after a build into a zip file that supports the FMU-standard.

## Usage
It will only make a FMU that supports the operating system that is currently running (should be improved later...)

For a debug build:

```
package_fmu_after_build
```

For a release build:

```
package_fmu_after_build --release
```
