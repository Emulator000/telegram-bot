use std::result;

pub use failure::Error;

pub type Result<T, E = Error> = result::Result<T, E>;
