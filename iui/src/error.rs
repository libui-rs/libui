//! Error types for this crate.

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
