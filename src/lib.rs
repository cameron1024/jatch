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
//! 
//! It can apply patches to JSON documents:
//!
//! ```rust
//! # use jatch::{apply, Patch, Path};
//! # use serde_json::json;
//! let doc = json!({"hello": "world"});
//! let patch = Patch::Add {
//!     path: Path::new("/foo"),
//!     value: json!("bar"),
//! };
//! assert_eq!(
//!     apply(doc, vec![patch]).unwrap(), 
//!     json!({
//!         "hello": "world",
//!         "foo": "bar",
//!     })
//! );
//! ```
//! It can also calculate the diffs between 2 JSONs:
//! ```rust
//! # use jatch::{diff, Patch, Path};
//! # use serde_json::json;
//! let patches = diff(
//!   json!({"hello": "world"}),  
//!   json!({"hello": "world", "foo": "bar"}),
//! );
//! assert_eq!(
//!     patches[0], 
//!     Patch::Add {
//!       path: Path::new("/foo"),
//!       value: json!("bar"),
//!     }
//! );
//! ```
mod errors;
mod patch;
mod path;
mod diff;

pub use errors::Error;
pub use patch::{
    apply::{apply_single, apply},
    Patch,
};
pub use path::Path;
pub use diff::diff;
