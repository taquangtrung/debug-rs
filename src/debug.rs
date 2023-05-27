//! Module to store mutable flags, used by all packages

/// Global variable to enable printing debugging information.
///
/// By default set to `true` to print debugging information in all unit tests.
pub static mut DEBUG: bool = true;

/// Global variable to enable printing extra debugging information.
///
/// By default set to `false` to hide the debugging information.
pub static mut DEBUG_EXTRA: bool = false;

/// Global variable which disables all printing functions.
pub static mut QUIET: bool = false;
