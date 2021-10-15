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
    clippy::perf
)]

//! A crate to provide a fast, correct and safe implementation of JSON Patch ([RFC 6902](https://datatracker.ietf.org/doc/html/rfc6902))
//!
//! It can apply patches to JSON documents:
//!
//! ```rust
//! # use jatch::{apply, Patch, Path, utils::*};
//! # use serde_json::json;
//! let doc = json!({"hello": "world"});
//! let patch = add("/foo", json!("bar")).unwrap();
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
//! # use jatch::{diff, Patch, Path, utils::*};
//! # use serde_json::json;
//! let patches = diff(
//!   json!({"hello": "world"}),  
//!   json!({"hello": "world", "foo": "bar"}),
//! );
//! assert_eq!(
//!     patches[0],
//!     add("/foo", json!("bar")).unwrap(),
//! );
//! ```

mod diff;
mod errors;
mod patch;
mod path;

pub use diff::diff;
pub use errors::Error;
pub use patch::walk::walk;
pub use patch::{
    apply::{apply, apply_single},
    Patch,
};
pub use path::Path;

/// A collection of helper functions for creating patches
pub mod utils {
    use std::convert::TryInto;

    use serde_json::Value;

    use crate::{Error, Patch, Path};

    /// A helper function to create an Add patch
    pub fn add(
        path: impl TryInto<Path, Error = Error>,
        value: impl Into<Value>,
    ) -> Result<Patch, Error> {
        Ok(Patch::Add {
            path: path.try_into()?,
            value: value.into(),
        })
    }

    /// A helper function to create a Remove patch
    pub fn remove(path: impl TryInto<Path, Error = Error>) -> Result<Patch, Error> {
        Ok(Patch::Remove {
            path: path.try_into()?,
        })
    }

    /// A helper function to create a Replace patch
    pub fn replace(
        path: impl TryInto<Path, Error = Error>,
        value: impl Into<Value>,
    ) -> Result<Patch, Error> {
        Ok(Patch::Replace {
            path: path.try_into()?,
            value: value.into(),
        })
    }

    /// A helper function to create a Move patch
    pub fn r#move(
        from: impl TryInto<Path, Error = Error>,
        path: impl TryInto<Path, Error = Error>,
    ) -> Result<Patch, Error> {
        Ok(Patch::Move {
            path: path.try_into()?,
            from: from.try_into()?,
        })
    }

    /// A helper function to create a Copy patch
    pub fn copy(
        from: impl TryInto<Path, Error = Error>,
        path: impl TryInto<Path, Error = Error>,
    ) -> Result<Patch, Error> {
        Ok(Patch::Copy {
            path: path.try_into()?,
            from: from.try_into()?,
        })
    }

    /// A helper function to create a Test patch
    pub fn test(
        path: impl TryInto<Path, Error = Error>,
        value: impl Into<Value>,
    ) -> Result<Patch, Error> {
        Ok(Patch::Test {
            path: path.try_into()?,
            value: value.into(),
        })
    }
}
