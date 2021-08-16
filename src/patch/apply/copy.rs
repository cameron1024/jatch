use serde_json::Value;

use crate::{Path, errors::Error, patch::walk::walk};

use super::add::add;

pub fn copy(root: Value, from: Path, path: Path) -> Result<Value, Error> {
    let value = walk(&root, from)?.clone();
    add(root, value, path)
}