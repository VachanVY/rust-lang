//! # pkgx
//! 
//! `pkgX` is a collection of utilities to make performing certain calculations more convenient.

pub use lib::add;

/// Adds two numbers
/// 
/// # Arguments
/// 
/// * `a` - The first number
/// * `b` - The second number
/// 
/// # Example
/// 
/// ```
/// let result = pkgx::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub mod lib {
    pub fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
}