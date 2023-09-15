// Import necessary modules and symbols for your crate.
pub mod rendering; // This module is public

// Re-export public symbols from your sub-modules.
pub use rendering::run; // This function is public

// Other public modules and symbols can be added here.
