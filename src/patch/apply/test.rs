use serde_json::Value;

use crate::{errors::Error, patch::walk::walk, Path};

pub fn test(root: Value, value: Value, path: Path) -> Result<Value, Error> {
    if walk(&root, path)? == &value {
        Ok(root)
    } else {
        Err(Error::FailedTest)
    }
}
#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::patch::{apply::apply_single, Patch};

    use super::*;

    #[test]
    fn test_test_success() {
        let test = Patch::Test {
            value: json!(123),
            path: Path::new("/a/b"),
        };

        let root = json!({
            "a": {
                "b": 123,
                "c": 234,
            },
            "b": [1],
        });
        apply_single(root, test).unwrap();
    }

    #[test]
    fn test_test_failure_incorrect_value() {
        let test = Patch::Test {
            value: json!(123),
            path: Path::new("/a/c"),
        };

        let root = json!({
            "a": {
                "b": 123,
                "c": 234,
            },
            "b": [1],
        });

        assert!(matches!(apply_single(root, test), Err(Error::FailedTest)));
    }

    #[test]
    fn test_test_failure_missing_path() {
        let test = Patch::Test {
            value: json!(123),
            path: Path::new("/missing"),
        };

        let root = json!({
            "a": {
                "b": 123,
                "c": 234,
            },
            "b": [1],
        });

        assert!(matches!(apply_single(root, test), Err(Error::PathDoesntExist)));
    }
}
