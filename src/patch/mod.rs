pub mod apply;
pub mod walk;

use crate::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A Json Patch operation
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(tag = "op")]
pub enum Patch {
    /// Inserts `value` into the location in the document referred to by `path`
    #[serde(rename = "add")]
    Add {
        /// The path to add to
        /// If the path does not exist, but its parent does, the path will be created
        /// If the parent of this path doesn't exist, the operation will fail
        path: Path,
        /// The value to add
        value: Value,
    },
    /// Remove the value at a location in the document referred to by `path`
    #[serde(rename = "remove")]
    Remove {
        /// The path to remove from
        /// If the path doesn't exist, the operation will fail
        path: Path,
    },
    /// Replace the contents of `path` with `value`
    /// This is equivalent to a `Replace` then an `Add`
    #[serde(rename = "replace")]
    Replace {
        /// The path to replace
        path: Path,
        /// The value to replace
        value: Value,
    },
    /// Copy the value at `from` to the location referred to by `path`
    #[serde(rename = "copy")]
    Copy {
        /// The path to copy from
        from: Path,
        /// The path to copy to
        path: Path,
    },
    /// Move the value at `from` to the location referred to by `path`
    /// Equivalent to a `Copy` followed by a `Remove`
    #[serde(rename = "move")]
    Move {
        /// The path to move from
        from: Path,
        /// the path to move to
        path: Path,
    },
    /// Check that the location referred to by `path` matches `value`
    /// If the the document does not contain `value` at `path`, an [Error::FailedTest] is returned
    #[serde(rename = "test")]
    Test {
        /// The path to test
        path: Path,
        /// The value expected at `path`
        value: Value,
    },
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::utils::{add, copy, r#move, remove, replace, test};

    use super::*;

    #[test]
    fn should_deserialize_patches() {
        use serde_json::from_str;

        assert_eq!(
            from_str::<Patch>(r#"{"op": "add", "path": "/foo", "value": "hello"}"#).unwrap(),
            add("/foo", json!("hello")).unwrap(),
        );

        assert_eq!(
            from_str::<Patch>(r#"{"op": "remove", "path": "/foo"}"#).unwrap(),
            remove("/foo").unwrap(),
        );

        assert_eq!(
            from_str::<Patch>(r#"{"op": "replace", "path": "/foo", "value": 123}"#).unwrap(),
            replace("/foo", json!(123)).unwrap(),
        );

        assert_eq!(
            from_str::<Patch>(r#"{"op": "copy", "from": "/foo", "path": "/to"}"#).unwrap(),
            copy("/foo", "/to").unwrap(),
        );

        assert_eq!(
            from_str::<Patch>(r#"{"op": "move", "from": "/foo", "path": "/to"}"#).unwrap(),
            r#move("/foo", "/to").unwrap()
        );

        assert_eq!(
            from_str::<Patch>(r#"{"op": "test", "path": "/foo", "value": true}"#).unwrap(),
            test("/foo", json!(true)).unwrap(),
        );
    }
}
