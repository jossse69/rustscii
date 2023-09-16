// Import necessary modules and symbols for your crate.
pub mod rendering; // This module is public

// Re-export public symbols from your sub-modules.
pub use rendering::run; // This function is public
pub use rendering::terminal::Terminal;
// Other public modules and symbols can be added here.
