use super::*;
use crate::core::*;
use crate::{core::Node, json::style::StyleJson};
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
    pub fn create(s: &str) -> Result<Self, TaffyBlueprintError> {
        match serde_json5::from_str::<Self>(s) {
            Ok(l) => Ok(l),
            Err(e) => Err(TaffyBlueprintError::Json(format!(
                "Error LayoutJson: {}",
                e
            ))),
        }
    }
}

struct Private(pub DrawTag);
impl TryFrom<String> for Private {
    type Error = TaffyBlueprintError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "" {
            return Ok(Private(DrawTag {
                position: DrawTagPosition::Center,
                color: "".into(),
                info: DrawTagText::None,
            }));
        }
        if let Some(caps) = RE.captures(&value) {
            let position = serde_json5::from_str(&format!("\"{}\"", &caps["position"]));
            match position {
                Ok(position) => Ok(Private(DrawTag {
                    position,
                    color: caps["color"].into(),
                    info: match caps["info"].into() {
                        "*" => DrawTagText::Id,
                        "" => DrawTagText::None,
                        s => DrawTagText::Text(s.into()),
                    },
                })),
                Err(_) => Err(TaffyBlueprintError::Json("Invalid DebugLabel".into())),
            }
        } else {
            Err(TaffyBlueprintError::Json("Invalid DebugLabel".into()))
        }
    }
}

impl Node<DrawTag> {
    fn children(children: Vec<NodeJson>) -> Result<Vec<Self>, TaffyBlueprintError> {
        children.into_iter().map(|c| c.try_into()).collect()
    }
    fn debug_label(value: NodeJson) -> Result<Self, TaffyBlueprintError> {
        let r: Result<Self, TaffyBlueprintError> = NodeJson {
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
                    Err(e) => match e {
                        TaffyBlueprintError::Json(msg) => {
                            println!("Warning! debug label not valid: {}", msg);
                            Ok(u)
                        }
                        _ => Err(e),
                    },
                }
            }
            Err(e) => Err(e),
        }
    }
}
impl TryFrom<NodeJson> for Node<DrawTag> {
    type Error = TaffyBlueprintError;

    fn try_from(value: NodeJson) -> Result<Self, Self::Error> {
        if !value.id.is_empty() {
            if !value.children.is_empty() {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        Self::children(value.children).map(|children| {
                            Node::Layout(value.id, value.style.unwrap().into(), children)
                        })
                    }
                } else {
                    Self::children(value.children).map(|children| Node::Id(value.id, children))
                }
            } else {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        Ok(Node::Leaf(value.id, value.style.unwrap().into()))
                    }
                } else {
                    Err(TaffyBlueprintError::Json(
                        "single id is not handled. Add at least a style or some children".into(),
                    ))
                }
            }
        } else {
            if !value.children.is_empty() {
                if value.style.is_some() {
                    if !value.debug_label.is_empty() {
                        Self::debug_label(value)
                    } else {
                        Self::children(value.children)
                            .map(|children| Self::Anonym(value.style.unwrap().into(), children))
                    }
                } else {
                    Self::children(value.children).map(|children| Self::fork(children))
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

#[cfg(test)]
mod tests {
    use taffy::prelude::*;

    use crate::json::json::json_to_node;

    use super::*;
    #[test]
    fn test_() {
        let lj = json_to_node(
            r#"
                {
                    "id": "a",
                    "style": {
                        "size": "123.4 234.5"
                    },
                    "children":[]
                }
        "#,
        );
        let l = Node::<DrawTag>::Leaf(
            "a".into(),
            Style {
                size: Size {
                    width: length(123.4),
                    height: length(234.5),
                },
                ..Default::default()
            },
        );
        assert_eq!(lj.unwrap(), l);
    }
}
