use super::*;
use crate::{
    core::{DebugLabel, Node},
    json::style::StyleJson,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<position>[^ ]+) (?<color>[^ ]+)( (?<info>[^ ]+))?").unwrap());

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct NodeJson {
    #[serde(default)]
    pub(crate) style: Option<StyleJson>,
    #[serde(default)]
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) children: Vec<Self>,
    #[serde(default)]
    pub(crate) debug_label: String,
}

impl NodeJson {
    pub fn create_json(s: &str) -> Result<Self, NodeJsonError> {
        match serde_json::from_str::<Self>(s) {
            Ok(l) => Ok(l),
            Err(e) => Err(NodeJsonError {
                msg: format!("Error LayoutJson: {}", e),
            }),
        }
    }
}

struct Private(pub DebugLabel);
impl TryFrom<String> for Private {
    type Error = NodeJsonError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "" {
            return Ok(Private(DebugLabel {
                position: DebugLabelPosition::Center,
                color: "".into(),
                info: DebugLabelText::None,
            }));
        }
        if let Some(caps) = RE.captures(&value) {
            let position = serde_json::from_str(&format!("\"{}\"", &caps["position"]));
            match position {
                Ok(position) => Ok(Private(DebugLabel {
                    position,
                    color: caps["color"].into(),
                    info: match caps["info"].into() {
                        "*" => DebugLabelText::Id,
                        "" => DebugLabelText::None,
                        s => DebugLabelText::Text(s.into()),
                    },
                })),
                Err(_) => Err(NodeJsonError {
                    msg: "Invalid DebugLabel".into(),
                }),
            }
        } else {
            Err(NodeJsonError {
                msg: "Invalid DebugLabel".into(),
            })
        }
    }
}

impl Node<DebugLabel> {
    fn children(children: Vec<NodeJson>) -> Result<Vec<Self>, NodeJsonError> {
        children.into_iter().map(|c| c.try_into()).collect()
    }
    fn debug_label(value: NodeJson) -> Result<Self, NodeJsonError> {
        let r: Result<Self, NodeJsonError> = NodeJson {
            id: value.id,
            style: value.style,
            children: value.children,
            debug_label: "".into(),
        }
        .try_into();
        match r {
            Ok(u) => {
                let v: Result<Private, _> = value.debug_label.try_into();
                match v {
                    Ok(v) => Ok(Node::Debug(Box::new(u), v.0)),
                    Err(e) => {
                        println!("Warning! debug label not valid: {}", e.msg);
                        Ok(u)
                    }
                }
            }
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<NodeJson> for Node<DebugLabel> {
    type Error = NodeJsonError;

    fn try_from(value: NodeJson) -> Result<Self, Self::Error> {
        if !value.id.is_empty() {
            if !value.children.is_empty() {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        match Self::children(value.children) {
                            Ok(children) => Ok(Node::Layout(
                                value.id,
                                value.style.unwrap().into(),
                                children,
                            )),
                            Err(e) => Err(e),
                        }
                    }
                } else {
                    match Self::children(value.children) {
                        Ok(children) => Ok(Node::Id(value.id, children)),
                        Err(e) => Err(e),
                    }
                }
            } else {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        Ok(Node::Leaf(value.id, value.style.unwrap().into()))
                    }
                } else {
                    Err(NodeJsonError {
                        msg: "id will be loosed. Add at least a style or some children".into(),
                    })
                }
            }
        } else {
            if !value.children.is_empty() {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        match Self::children(value.children) {
                            Ok(children) => Ok(Self::Anonym(value.style.unwrap().into(), children)),
                            Err(e) => Err(e),
                        }
                    }
                } else {
                    Err(NodeJsonError {
                        msg: "children will be loosed. Add at least an id or a style.".into(),
                    })
                }
            } else {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        Ok(Node::LeafAnonym(value.style.unwrap().into()))
                    }
                } else {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        Ok(Node::Empty)
                    }
                }
            }
        }
    }
}
