use crate::{core::Node, json::style::StyleJson};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct NodeJson {
    #[serde(default)]
    pub(crate) style: StyleJson,
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) children: Vec<Self>,
}
impl NodeJson {
    pub fn create_node(s: &str) -> Node {
        Self::create_json(s).map(|x| x.into()).unwrap_or_default()
    }
    pub fn create_json(s: &str) -> Option<Self> {
        match serde_json::from_str::<Self>(s) {
            Ok(l) => Some(l),
            Err(e) => {
                println!("Error LayoutJson: {}", e);
                None
            }
        }
    }
}

impl From<NodeJson> for Node {
    fn from(value: NodeJson) -> Self {
        Node::Layout(
            value.id,
            value.style.into(),
            value.children.into_iter().map(|c| c.into()).collect(),
        )
    }
}
