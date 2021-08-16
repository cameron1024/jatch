use std::fmt::Display;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_ref())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

/// An error encountered while performing JSON Patch operations
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    /// The provided string was not a valid JSON Pointer as defined in [RFC 6901](https://datatracker.ietf.org/doc/html/rfc6901)
    InvalidPath(String),
    /// The path given didn't refer to a valid location in the document
    PathDoesntExist,
    /// A JSON Patch 'test' operation failed
    FailedTest,
}
