use crate::combox_component::ComboBox;
use crate::update_result::UpdateResult;
use macroquad::window::{screen_height, screen_width};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"example_(?<name>[^ ]+).json").unwrap());

fn list_files_map(dir: impl AsRef<Path>) -> std::io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if let Some(caps) = RE.captures(name) {
                    map.insert(caps["name"].to_string(), path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(map)
}
pub struct JsonPickerComponent {
    jsons_map: HashMap<String, String>,
    combobox: ComboBox,
}

const JSONS_PATH: &str = "examples/json_hot_reloading/assets/jsons";

impl JsonPickerComponent {
    pub fn new() -> Self {
        Self {
            jsons_map: HashMap::new(),
            combobox: ComboBox::new(),
        }
    }
    pub fn start(&mut self) {
        match list_files_map(JSONS_PATH) {
            Ok(map) => {
                self.jsons_map = map;
            }
            Err(e) => {
                println!("WARNING: No json loaded: {}", e);
            }
        }
    }

    pub fn draw(&self) {
        self.combobox.draw();
    }
    pub fn update(&mut self) -> UpdateResult<String> {
        let (x, y) = if let Some(combobox_rect) = self.combobox.component_rect {
            let layout = TaffyNode::from_layout_node(Node::<()>::Layout(
                "root".into(),
                Style {
                    size: Size {
                        width: length(screen_width()),
                        height: length(screen_height()),
                    },
                    flex_direction: FlexDirection::Row,
                    justify_content: Some(JustifyContent::End),
                    align_items: Some(AlignItems::Start),
                    ..Default::default()
                },
                vec![Node::Anonym(
                    Style {
                        margin: Rect {
                            left: length(20.0),
                            right: length(20.0),
                            top: length(20.0),
                            bottom: length(20.0),
                        },
                        ..Default::default()
                    },
                    vec![Node::Leaf(
                        "combobox".into(),
                        Style {
                            size: Size {
                                width: length(combobox_rect.w),
                                height: length(combobox_rect.h),
                            },
                            ..Default::default()
                        },
                    )],
                )],
            ))
            .unwrap();
            let ref combobox_layout = layout.children["combobox"];
            let x = combobox_layout.layout.location.x + combobox_layout.absolute_position.x;
            let y = combobox_layout.layout.location.y + combobox_layout.absolute_position.y;
            (x, y)
        } else {
            (0.0, 0.0)
        };

        match self.combobox.update(
            self.jsons_map.iter().map(|(k, _)| k.clone()).collect(),
            16,
            x,
            y,
        ) {
            UpdateResult::Continue => UpdateResult::Continue,
            UpdateResult::End(text) => UpdateResult::End(self.jsons_map[&text].clone()),
        }
    }
}
