use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};

lazy_static! {
    static ref VERBOSE: AtomicBool = AtomicBool::new(false);
}

/// Sets the global verbose flag.
///
/// # Arguments
///
/// * `verbose` - If true, enables verbose mode; otherwise disables it.
pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, Ordering::Relaxed);
    print_verbose("set up '-v' environment")
}

/// Returns the current value of the global verbose flag.
///
/// # Returns
///
/// * `bool` indicating whether verbose mode is enabled.
pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

/// Prints a message if verbose mode is enabled.
///
/// # Arguments
///
/// * `message` - The message to print if verbose mode is enabled.
pub fn print_verbose(message: &str) {
    if is_verbose() {
        println!("[VERBOSE] {}", message);
    }
}
