use crate::{json::style::StyleJson, prelude::LayoutNode};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct LayoutJson {
    #[serde(default)]
    pub(crate) style: StyleJson,
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) children: Vec<Self>,
}
impl LayoutJson {
    pub fn create_node(s: &str) -> LayoutNode {
        Self::create_json(s).map(|x| x.into()).unwrap_or_default()
    }
    pub fn create_json(s: &str) -> Option<Self> {
        // let cfg: Config = json5::from_str(data).unwrap();
        match serde_json::from_str::<Self>(s) {
            Ok(l) => Some(l),
            Err(e) => {
                println!("Error LayoutJson: {}", e);
                None
            }
        }
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
