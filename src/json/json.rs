use super::*;
pub fn json_to_node(s: &str) -> Result<Node<DrawTag>, TaffyBlueprintError> {
    match NodeJson::create_json(s) {
        Ok(j) => j.try_into(),
        Err(e) => Err(e),
    }
}
