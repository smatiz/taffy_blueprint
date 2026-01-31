use super::*;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DebugLabel {
    pub position: DebugLabelPosition,
    pub color: String,
    pub info: DebugLabelText,
}

#[derive(Default, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum DebugLabelText {
    #[default]
    None,
    Id,
    Text(String),
}
#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum DebugLabelPosition {
    #[default]
    Center,
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

impl DebugLabelPosition {
    pub fn place(&self, width: f32, height: f32) -> Node<DebugLabel> {
        let (jc, ai) = match self {
            DebugLabelPosition::Center => (JustifyContent::Center, AlignItems::Center),
            DebugLabelPosition::N => (JustifyContent::Start, AlignItems::Center),
            DebugLabelPosition::S => (JustifyContent::End, AlignItems::Center),
            DebugLabelPosition::W => (JustifyContent::Center, AlignItems::Start),
            DebugLabelPosition::E => (JustifyContent::Center, AlignItems::End),
            DebugLabelPosition::NE => (JustifyContent::Start, AlignItems::Start),
            DebugLabelPosition::NW => (JustifyContent::Start, AlignItems::Center),
            DebugLabelPosition::SE => (JustifyContent::End, AlignItems::End),
            DebugLabelPosition::SW => (JustifyContent::End, AlignItems::Start),
        };

        Node::Anonym(
            Style {
                flex_direction: taffy::FlexDirection::Row,
                justify_content: Some(jc),
                align_items: Some(ai),
                ..Default::default()
            },
            vec![Node::Leaf(
                "debug".into(),
                Style {
                    size: Size {
                        width: length(width),
                        height: length(height),
                    },
                    ..Default::default()
                },
            )],
        )
    }
}
