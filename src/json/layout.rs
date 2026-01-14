use crate::{json::style::StyleJson, prelude::LayoutNode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct LayoutJson {
    pub(crate) style: StyleJson,
    pub(crate) id: String,
    pub(crate) children: Vec<Self>,
}
impl LayoutJson {
    pub fn create_node(s: &str) -> LayoutNode {
        serde_json::from_str::<Self>(s)
            .map(|x| x.into())
            .unwrap_or_default()
    }
}

impl From<LayoutJson> for LayoutNode {
    fn from(value: LayoutJson) -> Self {
        Self {
            id: value.id,
            style: value.style.into(),
            children: value.children.into_iter().map(|c| c.into()).collect(),
        }
    }
}
