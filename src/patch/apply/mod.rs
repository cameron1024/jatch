mod add;
mod copy;
mod r#move;
mod remove;
mod replace;
mod test;

use serde_json::Value;

use super::Patch;
use crate::errors::Error;

/// Applies a single JSON Patch to a JSON document
/// 
/// For example:
/// ```rust
/// # use jatch::{Patch, Path, apply_single, utils::*};
/// # use serde_json::json;
/// let root = json!({"foo": "bar"});
/// let patch = add("/hello", json!("world")).unwrap();
/// let root = apply_single(root, patch).unwrap();
/// assert_eq!(root, json!({
///    "foo": "bar",
///    "hello": "world",
/// }));
/// ```
pub fn apply_single(root: Value, patch: Patch) -> Result<Value, Error> {
    match patch {
        Patch::Add { value, path } => add::add(root, value, path),
        Patch::Remove { path } => remove::remove(root, path),
        Patch::Replace { value, path } => replace::replace(root, value, path),
        Patch::Copy { from, path } => copy::copy(root, from, path),
        Patch::Move { from, path } => r#move::r#move(root, from, path), // 'move' is a keyword
        Patch::Test { value, path } => test::test(root, value, path),
    }
}

/// Applies a collection of JSON Patches to a JSON document
/// The patches are applied in order, and if any individual patch fails, the whole function fails
/// 
/// For example:
/// ```rust
/// # use jatch::{Patch, Path, apply, utils::*};
/// # use serde_json::json;
/// let root = json!({"foo": "bar"});
/// let patches = vec![
///   add("/hello", json!("world")).unwrap(),
///   remove("/foo").unwrap(),
/// ];
/// let root = apply(root, patches).unwrap();
/// assert_eq!(root, json!({
///    "hello": "world",
/// }));
/// ```
pub fn apply(
    mut root: Value,
    patches: impl IntoIterator<Item = Patch>,
) -> Result<Value, Error> {
    for patch in patches {
        root = apply_single(root, patch)?;
    }
    Ok(root)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    use serde::Deserialize;
    use serde_json::{from_str, from_value};

    #[derive(Deserialize, Debug, Clone)]
    struct TestCase {
        doc: Value,
        patch: Vec<Patch>,
        expected: Option<Value>,
        error: Option<String>,
        comment: Option<String>,
    }

    fn load_tests() -> Vec<Value> {
        let s1 = read_to_string("testing/tests.json").unwrap();
        let s2 = read_to_string("testing/spec_tests.json").unwrap();
        let mut v = from_str::<Vec<Value>>(&s1).unwrap();
        v.extend(from_str::<Vec<Value>>(&s2).unwrap());
        v
    }

    fn test_single(test_json: Value, index: usize) {
        if test_json.get("expected").is_some() {
            assert!(did_test_succeed(test_json, index));
        } else {
            assert!(!did_test_succeed(test_json, index));
        }
    }

    // this could maybe do with a refactor
    fn did_test_succeed(test_json: Value, index: usize) -> bool {
        if let Some(expected) = test_json.get("expected") {
            let expected = expected.clone();
            if let Some(root) = test_json.get("doc") {
                let root = root.clone();
                if let Some(patch) = test_json.get("patch") {
                    if let Ok(patch) = from_value::<Vec<Patch>>(patch.clone()) {
                        let actual = apply(root, patch);
                        println!("running test for {}, number {}", test_json, index);
                        let actual = actual.map(|value| assert_eq!(value, expected));
                        actual.is_ok()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    #[test]
    fn test_cases() {
        load_tests()
            .iter()
            .enumerate()
            .for_each(|(index, value)| test_single(value.to_owned(), index));
    }
}
