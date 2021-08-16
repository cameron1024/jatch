use serde_json::Value;

use crate::{errors::Error, Path};

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
    s.as_ref().parse::<usize>().or_else(|_| {
        if s.as_ref() == "-" {
            Ok(vec.len())
        } else {
            Err(Error::InvalidPath(format!(
                "Expected array index, got '{}'",
                s.as_ref(),
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
    fn should_walk_json() {
        assert_eq!(
            walk(&default_json(), Path::new("/a")).unwrap(),
            &json!("abc")
        );
        assert_eq!(walk(&default_json(), Path::new("/b")).unwrap(), &json!(123));
        assert_eq!(
            walk(&default_json(), Path::new("/c")).unwrap(),
            &json!(true)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/d")).unwrap(),
            &json!(null)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/e")).unwrap(),
            &json!([1, 2, 3])
        );
        assert_eq!(walk(&default_json(), Path::new("/e/0")).unwrap(), &json!(1));
        assert_eq!(walk(&default_json(), Path::new("/e/2")).unwrap(), &json!(3));
        assert_eq!(
            walk(&default_json(), Path::new("/f")).unwrap(),
            &json!({"a": "abc", "b": 123,})
        );
        assert_eq!(
            walk(&default_json(), Path::new("/f/a")).unwrap(),
            &json!("abc")
        );
        assert_eq!(
            walk(&default_json(), Path::new("/f/b")).unwrap(),
            &json!(123)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/x/z")),
            Err(Error::PathDoesntExist)
        );
        assert_eq!(
            walk(&default_json(), Path::new("/e/-")),
            Err(Error::PathDoesntExist)
        );
        assert_eq!(
            walk(&default_json(), Path::root()).unwrap(),
            &default_json()
        )
    }
}
