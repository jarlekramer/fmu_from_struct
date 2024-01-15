# Utilities to run in the build process of an FMU

Utilities written in Python to use together with Cargo when building FMUs using Fmrs_model.

## Install instructions
`
pip install .
`

## Main use case
Run the post-build script after cargo build:
`
python -m fmrs_model_build_utils.post_build_script --name fmu_name --version fmi_version
`