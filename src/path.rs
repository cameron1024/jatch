use core::panic;

use serde::{de::Visitor, Deserialize, Serialize};

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
        Ok(Path::new(v))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Path::new(v))
    }
}

const TILDE_ESCAPE: &str = "~0";
const SLASH_ESCAPE: &str = "~1";

impl Path {
    /// Create a new [Path] from an escaped string, as defined in [RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901)
    pub fn new(s: impl AsRef<str>) -> Self {
        if s.as_ref() == "" {
            return Self::root();
        };

        if !s.as_ref().starts_with('/') {
            panic!("invalid json path: {}", s.as_ref())
        }

        Self {
            parts: s.as_ref().split('/').skip(1).map(Self::escape).collect(),
        }
    }

    /// Append a path to this path
    /// A leading slash is added to the path
    pub fn join(mut self, s: impl AsRef<str>) -> Self {
        let other = Path::new(format!("/{}", s.as_ref()));
        self.parts.extend(other.parts);
        self
    }

    fn escape(s: impl AsRef<str>) -> String {
        let s = s.as_ref().replace(SLASH_ESCAPE, "/");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_split_paths_by_slash() {
        assert_eq!(Path::new("/foo").parts, vec!["foo"]);
        assert_eq!(Path::new("/foo/bar").parts, vec!["foo", "bar"]);
        assert_eq!(Path::new("/").parts, vec![""]);
        assert_eq!(Path::new("//foo").parts, vec!["", "foo"]);
        assert_eq!(Path::new("").parts, Vec::<String>::new());
    }

    #[test]
    fn should_escape_properly() {
        assert_eq!(Path::new("/~0").parts, vec!["~"]);
        assert_eq!(Path::new("/~1").parts, vec!["/"]);
        assert_eq!(Path::new("/~01").parts, vec!["~1"]);
    }

    #[test]
    fn root_should_equal_empty_string() {
        assert_eq!(Path::new(""), Path::root());
    }

    #[test]
    fn escape_unescape_round_trip() {
        let paths = vec!["", "/", "/~0", "/~1", "/~01", "/hello/~0asdf~1/world"];
        for path in paths {
            assert_eq!(path, &Path::new(path).to_escaped());
        }
    }
}
