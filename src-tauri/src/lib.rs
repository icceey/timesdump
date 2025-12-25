// Library module for Timesdump
// This module will contain the core business logic

pub mod error {
    pub use anyhow::{Result, Error, Context};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert_eq!(1 + 1, 2);
    }
}
