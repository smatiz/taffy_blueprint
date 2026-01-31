use super::*;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DrawTag {
    pub position: DrawTagPosition,
    pub color: String,
    pub info: DrawTagText,
}

#[derive(Default, Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum DrawTagText {
    #[default]
    None,
    Id,
    Text(String),
}
#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum DrawTagPosition {
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

impl DrawTagPosition {
    pub fn node(
        &self,
        rect_width: f32,
        rect_height: f32,
        text_width: f32,
        text_height: f32,
    ) -> Node<DrawTag> {
        let (jc, ai) = match self {
            DrawTagPosition::Center => (JustifyContent::Center, AlignItems::Center),
            DrawTagPosition::N => (JustifyContent::Center, AlignItems::Start),
            DrawTagPosition::S => (JustifyContent::Center, AlignItems::End),
            DrawTagPosition::W => (JustifyContent::Start, AlignItems::Center),
            DrawTagPosition::E => (JustifyContent::End, AlignItems::Center),
            DrawTagPosition::NE => (JustifyContent::Start, AlignItems::Start),
            DrawTagPosition::NW => (JustifyContent::Center, AlignItems::Start),
            DrawTagPosition::SE => (JustifyContent::End, AlignItems::End),
            DrawTagPosition::SW => (JustifyContent::Start, AlignItems::End),
        };

        Node::Layout(
            "node".into(),
            Style {
                size: Size {
                    width: length(rect_width),
                    height: length(rect_height),
                },
                flex_direction: taffy::FlexDirection::Row,
                justify_content: Some(jc),
                align_items: Some(ai),
                ..Default::default()
            },
            vec![Node::Leaf(
                "debug".into(),
                Style {
                    size: Size {
                        width: length(text_width),
                        height: length(text_height),
                    },
                    ..Default::default()
                },
            )],
        )
    }
}
