use serde_json::Value;

use crate::{errors::Error, patch::walk::parse_array_index, Path};

pub fn add(mut root: Value, value: Value, path: Path) -> Result<Value, Error> {
    if let Some((head, tail)) = path.split_head() {
        // perform the update in place
        match &mut root {
            Value::Object(ref mut map) => {
                // if there is no more path left, we can just insert into the map
                if tail.is_empty() {
                    map.insert(head, value);
                } else {
                    // we need to go deeper, check for inner value, update if it exists, or error if it is missing
                    if let Some(mut inner_value) = map.remove(&head) {
                        inner_value = add(inner_value, value, tail)?;
                        map.insert(head, inner_value);
                    } else {
                        return Err(Error::PathDoesntExist);
                    }
                }
            }
            Value::Array(ref mut vec) => {
                let head_index = parse_array_index(vec, &head)?;
                // general logic is very similar to above

                // there is no more path, just insert at the right index
                if tail.is_empty() {
                    vec.insert(head_index, value);
                } else {
                    // go deeper, check for value at the index, update if it exists, err if its missing
                    if head_index < vec.len() {
                        let mut inner_value = vec.remove(head_index);
                        inner_value = add(inner_value, value, tail)?;
                        vec.insert(head_index, inner_value);
                    } else {
                        return Err(Error::PathDoesntExist);
                    }
                }
            }
            _ => return Err(Error::PathDoesntExist),
        }
        // then return the instance
        Ok(root)
    } else {
        // if recursed, this is never hit, since we always check that the tail path is not empty
        // an "add" operation to the root of the document essentially "sets" the document to the provided value
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_add_object() {
        let root = json!({
            "hello": "123",
            "world": 123,
        });
        let value_to_add = json!("added");
        let path = Path::new("/new").unwrap();
        let new = add(root, value_to_add, path).unwrap();
        assert_eq!(new.get("new").unwrap(), &json!("added"));

        let new_again = add(new, json!(234), Path::new("/hello").unwrap()).unwrap();
        assert_eq!(new_again.get("hello").unwrap(), &json!(234));
    }

    #[test]
    fn test_deeply_nested_add() {
        let root = json!({
            "a": {
                "b": {
                    "c": {
                        "a": 1,
                        "b": 1,
                        "c": 1,
                    }
                }
            }
        });
        let value_to_add = json!(2);
        let path = Path::new("/a/b/c/d").unwrap();
        let new = add(root, value_to_add, path).unwrap();
        let deep_object = new.get("a").unwrap().get("b").unwrap().get("c").unwrap();
        assert_eq!(deep_object.get("d").unwrap(), &json!(2));
    }

    #[test]
    fn add_into_array_by_index() {
        let root = json!([1, 2, 3]);
        let value = json!(4);
        let root = add(root, value, Path::new("/0").unwrap()).unwrap();
        assert_eq!(root, json!([4, 1, 2, 3]));
    }

    #[test]
    fn add_into_array_with_hyphen() {
        let root = json!([1, 2, 3]);
        let value = json!(4);
        let root = add(root, value, Path::new("/-").unwrap()).unwrap();
        assert_eq!(root, json!([1, 2, 3, 4]));
    }

    #[test]
    fn deep_nesting_array_object() {
        let root = json!({
            "a": [
                1, 2, {
                    "b": [
                        3, 4, {
                            "c": "hello",
                            "arr": [1, 2, 3]
                        }
                    ]
                }
            ]
        });
        let new = add(root, json!(123), Path::new("/a/2/b/2/arr/-").unwrap()).unwrap();
        let deep_array = new
            .get("a")
            .unwrap()
            .get(2)
            .unwrap()
            .get("b")
            .unwrap()
            .get(2)
            .unwrap()
            .get("arr")
            .unwrap();
        assert_eq!(deep_array, &json!([1, 2, 3, 123]));
    }
}
