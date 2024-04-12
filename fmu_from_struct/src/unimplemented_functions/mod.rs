
//! Placeholder for functions not yet properly implemented in the derive macro.
//! 
//! At least some fmu-simulators require all functions in the fmi standard to have an implementation,
//! even though they may not be used. For instance, running a co-simulation does not really need all 
//! the model exchange functions, but it is impossible to load an fmu in fmpy without them.
//! 
//! This module therefore implements dummy versions for missing functions, so that the fmu can be 
//! loaded. IMPORTANT: as none of the functions actually do anything, they will not work as intended 
//! for cases where they are needed.  


/// Dummy implementations for fmi2 functions
pub mod fmi2;

/// Dummy implementations for fmi3 functions
pub mod fmi3;
