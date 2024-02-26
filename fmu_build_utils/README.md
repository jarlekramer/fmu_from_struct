# Utilities to run in the build process of an FMU

Utilities written in Python to use together with Cargo when building FMUs using `fmu_from_struct`.

## Install Instructions
```
pip install .
```

## Main use case
Running the script below packages an FMU from the output from Cargo:

```
python -m fmu_build_utils.package_fmu
```