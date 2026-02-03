use crate::core::*;
use serde::{Deserialize, Serialize};
use taffy::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DrawTag {
    pub position: DrawTagPosition,
    pub color: String,
    pub info: DrawTagText,
}

#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum DrawTagText {
    #[default]
    None,
    Id,
    Text(String),
}

// impl std::fmt::Debug for DrawTagText {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::None => write!(f, "None"),
//             Self::Id => write!(f, "Id"),
//             Self::Text(arg0) => f.debug_tuple("Text").field(arg0).finish(),
//         }
//     }
// }

#[derive(Default, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum DrawTagPosition {
    #[default]
    None,
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
    pub fn location(
        &self,
        rect_width: f32,
        rect_height: f32,
        text_width: f32,
        text_height: f32,
    ) -> Result<Option<(f32, f32)>, TaffyBlueprintError> {
        if let Some((jc, ai)) = match self {
            DrawTagPosition::Center => Some((JustifyContent::Center, AlignItems::Center)),
            DrawTagPosition::N => Some((JustifyContent::Center, AlignItems::Start)),
            DrawTagPosition::S => Some((JustifyContent::Center, AlignItems::End)),
            DrawTagPosition::W => Some((JustifyContent::Start, AlignItems::Center)),
            DrawTagPosition::E => Some((JustifyContent::End, AlignItems::Center)),
            DrawTagPosition::NW => Some((JustifyContent::Start, AlignItems::Start)),
            DrawTagPosition::NE => Some((JustifyContent::End, AlignItems::Start)),
            DrawTagPosition::SW => Some((JustifyContent::Start, AlignItems::End)),
            DrawTagPosition::SE => Some((JustifyContent::End, AlignItems::End)),
            DrawTagPosition::None => None,
        } {
            let node = Node::<Self>::Layout(
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
            );

            TaffyNode::from_layout_node(node).and_then(|taffy_node| {
                let d = &taffy_node.children["debug"];
                Ok(Some((d.layout.location.x, d.layout.location.y)))
            })
        } else {
            Ok(None)
        }
    }
}
