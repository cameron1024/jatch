use serde_json::Value;

use crate::{errors::Error, Path};

/// Reads the value at a point in a JSON document
///
/// '''
/// # use serde_json::json;
/// # use serde_json::Value;
/// let value = json!({"a": "b", "c": "d"});
/// let c_value = walk(&json, Path::new("/c").unwrap()).unwrap();
/// assert_eq!(c_value, json!("d"));
/// '''
pub fn walk(value: &Value, path: Path) -> Result<&Value, Error> {
    if let Some((head, tail)) = path.split_head() {
        match value {
            Value::Object(map) => map
                .get(&head)
                .ok_or(Error::PathDoesntExist)
                .and_then(|value| walk(value, tail)),
            Value::Array(vec) => {
                let index = parse_array_index(vec, head)?;
                walk(vec.get(index).ok_or(Error::PathDoesntExist)?, tail)
            }
            _ => Err(Error::PathDoesntExist),
        }
    } else {
        Ok(value)
    }
}

pub fn parse_array_index<T>(vec: &[T], s: impl AsRef<str>) -> Result<usize, Error> {
    let s = s.as_ref();
    s.parse::<usize>().or_else(|_| {
        if s == "-" {
            Ok(vec.len())
        } else {
            Err(Error::InvalidPath(format!(
                "Expected array index, got '{}'",
                s,
            )))
        }
    })
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    fn default_json() -> Value {
        json!({
            "a": "abc",
            "b": 123,
            "c": true,
            "d": null,
            "e": [1, 2, 3],
            "f": {
                "a": "abc",
                "b": 123
            }
        })
    }

    #[test]
    fn should_parse_array_indices() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(parse_array_index(&v, "0").unwrap(), 0);
        assert_eq!(parse_array_index(&v, "1").unwrap(), 1);
        assert_eq!(parse_array_index(&v, "2").unwrap(), 2);
        assert_eq!(parse_array_index(&v, "-").unwrap(), 5);
    }

    #[test]
    fn should_walk_json() {
        assert_eq!(
            walk(&default_json(), Path::new("/a").unwrap()).unwrap(),
            &json!("abc")
        );
        assert_eq!(
            walk(&default_json(), Path::new("/b").unwrap()).unwrap(),
            &json!(123)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/c").unwrap()).unwrap(),
            &json!(true)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/d").unwrap()).unwrap(),
            &json!(null)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/e").unwrap()).unwrap(),
            &json!([1, 2, 3])
        );
        assert_eq!(
            walk(&default_json(), Path::new("/e/0").unwrap()).unwrap(),
            &json!(1)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/e/2").unwrap()).unwrap(),
            &json!(3)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/f").unwrap()).unwrap(),
            &json!({"a": "abc", "b": 123,})
        );
        assert_eq!(
            walk(&default_json(), Path::new("/f/a").unwrap()).unwrap(),
            &json!("abc")
        );
        assert_eq!(
            walk(&default_json(), Path::new("/f/b").unwrap()).unwrap(),
            &json!(123)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/x/z").unwrap()),
            Err(Error::PathDoesntExist)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/e/-").unwrap()),
            Err(Error::PathDoesntExist)
        );
        assert_eq!(
            walk(&default_json(), Path::root()).unwrap(),
            &default_json()
        )
    }
}
