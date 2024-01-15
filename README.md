# fmrs_model
Library for setting up an FMU model in rust

## Purpose
Automate the implementaiton of a functional mockup interface for models written in rust.

Co-simulation is seen as the primary use case for this macro, and currently the only supported type 
of implementation. Model exchange may be added in the future if there is a need / interest from 
users. 

Only version 2 and 3 of the fmi standard is supported, with no plans to add support for version 1. 
Version 2 is needed as version 3 is yet to be widely used. For instance, the [open simulation 
platform](https://opensimulationplatform.com/) is still based on version 2. Version 3 is expected to
be adopted more and more in the near future. 
