//! # Common library
//!
//! `common` is a collection of common utilities for the Rust programming language.

/// Adds two number
///
/// # Examples
///
/// ```
/// use rand::random;
///
/// let left = random::<usize>() % 10;
/// let result = common::add(left, 2);
/// assert_eq!(result, left + 2);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
