use std::convert::TryFrom;

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::Error;

/// A reference to location in a JSON document, as defined in [RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    parts: Vec<String>,
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_escaped().as_ref())
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MyVisitor)
    }
}

struct MyVisitor;
impl<'a> Visitor<'a> for MyVisitor {
    type Value = Path;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a json patch")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Path::new(v).map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v), &""))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Path::new(&v).map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(v.as_ref()), &""))
    }
}

const TILDE_ESCAPE: &str = "~0";
const SLASH_ESCAPE: &str = "~1";

impl Path {
    /// Create a new [Path] from an escaped string, as defined in [RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901)
    pub fn new(s: impl Into<String>) -> Result<Self, Error> {
        let s = s.into();
        if s.is_empty() {
            return Ok(Self::root());
        };

        if !s.starts_with('/') {
            return Err(Error::InvalidPath(s));
        }

        Ok(Self {
            parts: s.split('/').skip(1).map(Self::escape).collect(),
        })
    }

    /// Append a path to this path
    /// A leading slash is added to the path
    pub fn join(mut self, s: impl Into<String>) -> Self {
        let other = Path::new(format!("/{}", s.into()));
        self.parts.extend(other.unwrap().parts); // this unwrap is safe, the only possible error is if the path doesn't begin with a slash (and is non-empty) which is impossible here
        self
    }

    fn escape(s: impl Into<String>) -> String {
        let s = s.into().replace(SLASH_ESCAPE, "/");
        s.replace(TILDE_ESCAPE, "~")
    }

    /// Create a [Path] pointing to the root of the document
    pub fn root() -> Self {
        Self { parts: vec![] }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.parts.len()
    }

    pub(crate) fn split_head(mut self) -> Option<(String, Self)> {
        match &self.len() {
            0 => None,
            _ => {
                let head = self.parts.remove(0);
                Some((head, Self { parts: self.parts }))
            }
        }
    }

    pub(crate) fn to_escaped(&self) -> String {
        if self.is_empty() {
            String::from("")
        } else {
            let s = self
                .parts
                .iter()
                .map(|s| {
                    let s = s.replace('~', TILDE_ESCAPE);
                    s.replace('/', SLASH_ESCAPE)
                })
                .collect::<Vec<_>>()
                .join("/");
            format!("/{}", s)
        }
    }
}

impl TryFrom<&str> for Path {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Path {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_split_paths_by_slash() {
        assert_eq!(Path::new("/foo").unwrap().parts, vec!["foo"]);
        assert_eq!(Path::new("/foo/bar").unwrap().parts, vec!["foo", "bar"]);
        assert_eq!(Path::new("/").unwrap().parts, vec![""]);
        assert_eq!(Path::new("//foo").unwrap().parts, vec!["", "foo"]);
        assert_eq!(Path::new("").unwrap().parts, Vec::<String>::new());
    }

    #[test]
    fn should_escape_properly() {
        assert_eq!(Path::new("/~0").unwrap().parts, vec!["~"]);
        assert_eq!(Path::new("/~1").unwrap().parts, vec!["/"]);
        assert_eq!(Path::new("/~01").unwrap().parts, vec!["~1"]);
    }

    #[test]
    fn root_should_equal_empty_string() {
        assert_eq!(Path::new("").unwrap(), Path::root());
    }

    #[test]
    fn escape_unescape_round_trip() {
        let paths = vec!["", "/", "/~0", "/~1", "/~01", "/hello/~0asdf~1/world"];
        for path in paths {
            assert_eq!(path, &Path::new(path).unwrap().to_escaped());
        }
    }
}
