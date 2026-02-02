use super::*;
use crate::core::*;
pub fn json_to_node(s: &str) -> Result<Node<DrawTag>, TaffyBlueprintError> {
    match NodeJson::create(s) {
        Ok(j) => j.try_into(),
        Err(e) => Err(e),
    }
}
