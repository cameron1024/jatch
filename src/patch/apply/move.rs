use serde_json::Value;

use crate::{Path, errors::Error, patch::walk::walk};

use super::{add::add, remove::remove};

pub fn r#move(root: Value, from: Path, path: Path) -> Result<Value, Error> {
    if from == path {
        Ok(root)
    } else {
        let value_to_move = walk(&root, from.clone())?.clone();
        let removed = remove(root, from)?;
        add(removed, value_to_move, path)
    }
}
