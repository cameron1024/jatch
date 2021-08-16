#![deny(
    unsafe_code,
    missing_docs,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::perf,
)]
//! A crate to provide a fast, correct and safe implementation of JSON Patch ([RFC 6902](https://datatracker.ietf.org/doc/html/rfc6902))

mod errors;
mod patch;
mod path;

pub use errors::Error;
pub use patch::{
    apply::{apply, apply_all},
    Patch,
};
pub use path::Path;