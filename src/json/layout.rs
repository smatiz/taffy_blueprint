use super::*;

#[derive(Debug)]
pub struct NodeJsonError {
    pub msg: String,
}
impl std::fmt::Display for NodeJsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.msg.fmt(f)
    }
}

pub fn json_to_node(s: &str) -> Result<Node<DebugLabel>, NodeJsonError> {
    match NodeJson::create_json(s) {
        Ok(j) => j.try_into(),
        Err(e) => Err(e),
    }
}
