use serde_json::Value;

use crate::{Path, errors::Error};

use super::{add::add, remove::remove};


pub fn replace(root: Value, value: Value, path: Path) -> Result<Value, Error> {
  // potential to make this faster by doing it in a single pass
  let without_old_value = remove(root, path.clone())?;
  add(without_old_value, value, path)
}