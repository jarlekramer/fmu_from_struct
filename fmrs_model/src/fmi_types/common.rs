//! Types that are common to all fmi versions.

#[repr(C)]
/// Enum that represents the states that an fmi model can return. 
/// 
/// Note: technically, the fmi version 2 has an addtional field called "fmi2Pending" that is not
/// included here. Thas is beacuse it is currently not used in this macro, and it is very useful to 
/// have one enum rather than two different ones for each fmi version. This might change in the 
/// future if it is discovered to be necessary...
pub enum FmiStatus {
    Ok,
    Warning,
    Discard,
    Error,
    Fatal,
}