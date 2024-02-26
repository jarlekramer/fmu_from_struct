# fmu_from_struct
A derive crate for automatically setting up FMU models in Rust

## Development Status
**WARNING:** This crate is very much in development. Bugs can be expected in the current version and 
breaking changes are to be expected in future versions. The reason it is released openly is mostly 
to allow people interested in the purpose of this crate (see below) to contribute to and comment on 
the development.

Only the most basic functionality in the FMI standard is so far implemented. However, this is often 
enough for simple models. The library is therefore used to implement FMUs from Rust code in separate 
(closed source) projects related to maritime research. The basic functionality seems to work as 
intended.

## Purpose
Automate the implementation of a functional mockup interface for models written in Rust.

Co-simulation is seen as the primary use case for this macro and is currently the only supported 
type of implementation. Model exchange may be added in the future if there is a need or interest 
from users. 

Only versions 2 and 3 of the FMI standard are supported, with no plans to add support for version 1.
Due to how this code is used in other projects by the developer, version 2 is tested more than 
version 3, and may be considered the safer choice.  

Version 2 is needed as version 3 is yet to be widely used. For instance, the developer of this crate
often uses the [open simulation platform](https://opensimulationplatform.com/) to run simulations, 
and this is still based on version 2. Version 3 is expected to be the main version in the future 
(especially when the Open Simulation Platform changes its version)

## Getting Started
Check the `examples` folder for code examples on how to use this macro.

## Folder Structure
- `fmu_from_struct` contains the overall interface to the rust functionality. That is: traits and 
prelude imports.
- `fmu_from_struct_derive` contains the derive macro itself
- `examples` contains simple examples of how to use the macro
- `fmu_build_utils` contains a Python script to take the result from `Cargo build` and package it to 
an FMU zip file. This can also be done manually, but this script is intended to automate this task. 
**Note**: this functionality is likely to change in the future. Should probably rather be an 
executable written in Rust, that can be installed by Cargo. Also, as of now, it only works for 
Windows. Should be easy enough to fix for other platforms, but yet to be done.

## Contribute
If you would like to contribute, that's great! Please contact the repository owner for an informal 
discussion about how and what. 

If you only would like to raise an issue, feel free to create one in this repository. However, no 
promises on when it will be fixed...

## License
This macro is licensed under the MIT License. See [LICENSE](LICENSE) or 
https://opensource.org/license/MIT for details.