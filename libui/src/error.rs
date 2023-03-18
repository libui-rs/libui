//! Error types for this crate.

use std::{error::Error, fmt::Display};

/// The error type returned by functions in this crate which might fail.
#[derive(Debug)]
pub enum UIError {
    /// Signifies that the underlying library was unable to properly hook into the platform's GUI APIs.
    FailedInitError { error: String },
    /// Signifies that an attempt was made to initialize a new instance of the underlying library while
    /// one already existed.
    MultipleInitError(),
    /// Signifies that an attempt was made to remove a tab (index) from a tab group that was out of bounds (n).
    TabGroupIndexOutOfBounds { index: i32, n: i32 },
}

impl Display for UIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UIError::FailedInitError { error } => write!(
                f,
                "unable to initialize the underlying UI framework: {}",
                error
            ),
            UIError::MultipleInitError() => {
                write!(f, "cannot initialize multiple instances of libui")
            }
            UIError::TabGroupIndexOutOfBounds { index, n } => write!(
                f,
                "tab with index {} is not in tab group of size {}",
                index, n
            ),
        }
    }
}

impl Error for UIError {}
