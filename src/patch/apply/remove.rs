use serde_json::Value;

use crate::{errors::Error, patch::walk::parse_array_index, Path};

pub fn remove(mut root: Value, path: Path) -> Result<Value, Error> {
    if let Some((head, tail)) = path.split_head() {
        // modify the value in place
        match root {
            Value::Object(ref mut map) => {
                if tail.is_empty() {
                    map.remove(&head);
                } else {
                    let mut inner_map = map.remove(&head).ok_or(Error::PathDoesntExist)?;
                    inner_map = remove(inner_map, tail)?;
                    map.insert(head, inner_map);
                }
            }
            Value::Array(ref mut vec) => {
                let head_index = parse_array_index(vec, head)?;
                if head_index < vec.len() {
                    if tail.is_empty() {
                        vec.remove(head_index);
                    } else {
                        let mut inner_value = vec.remove(head_index);
                        inner_value = remove(inner_value, tail)?;
                        vec.insert(head_index, inner_value);
                    }
                } else {
                    return Err(Error::PathDoesntExist);
                }
            }
            _ => return Err(Error::PathDoesntExist),
        }
    }
    Ok(root)
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn remove_from_array() {
        let root = json!([1, 2, 3]);
        let without_first = remove(root, Path::new("/0").unwrap()).unwrap();
        assert_eq!(without_first, json!([2, 3]));

        let error = remove(without_first, Path::new("/-").unwrap()).unwrap_err();
        assert_eq!(error, Error::PathDoesntExist);
    }

    #[test]
    fn remove_from_object() {
        let root = json!({"a": 1, "b": 2});
        let without_a = remove(root, Path::new("/a").unwrap()).unwrap();
        assert_eq!(without_a, json!({"b": 2}));

        let error = remove(without_a, Path::new("/b/c").unwrap()).unwrap_err();
        assert_eq!(error, Error::PathDoesntExist);
    }

    #[test]
    fn remove_from_deep_array() {
        let root = json!([1, 2, [3, 4, [4, 5, 6]]]);
        let without_6 = remove(root, Path::new("/2/2/2").unwrap()).unwrap();
        assert_eq!(without_6, json!([1, 2, [3, 4, [4, 5]]]))
    }

    #[test]
    fn remove_from_deep_object() {
        let root = json!({
            "a": {
                "b": {
                    "c": {
                        "d": 1,
                        "e": 2,
                    }
                }
            }
        });
        let without_e = remove(root, Path::new("/a/b/c/e").unwrap()).unwrap();
        assert_eq!(
            without_e,
            json!({
                "a": {
                    "b": {
                        "c": {
                            "d": 1
                        }
                    }
                }
            })
        );
    }
}
