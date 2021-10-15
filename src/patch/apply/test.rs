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

    use crate::{patch::apply::apply_single, utils::test};

    use super::*;

    #[test]
    fn test_test_success() {
        let test = test("/a/b", json!(123)).unwrap();
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
        let test = test("/a/c", json!(123)).unwrap();
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
        let test = test("/missing", json!(123)).unwrap();
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
