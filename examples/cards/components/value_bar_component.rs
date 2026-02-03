use super::*;
use macroquad::prelude::*;
use taffy::prelude::*;
use taffy_blueprint::prelude::*;

const SIZE: usize = 6;

#[derive(Debug, Clone)]
pub struct ValueBarComponent {
    values: [ComponentId<ValueBarItemComponent>; SIZE],
}
impl ValueBarComponent {
    pub fn new(value: u8) -> Self {
        let values = (0..SIZE)
            .map(|i| {
                ComponentId::new(
                    &format!("{}", i),
                    ValueBarItemComponent::new(i < value as usize),
                )
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { values }
    }
}
impl Component for ValueBarComponent {
    fn draw(&self, text_drawer: &TextDrawer, rects: &TaffyRectNode<()>) {
        if let Some(rects) = rects.get_child("bar") {
            let rect = rects.rect();
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
            for comp in self.values.iter() {
                if let Some(rects) = rects.get_child(&comp.id) {
                    comp.item.draw(text_drawer, rects);
                }
            }
        }
    }
    fn layout(&self, text_drawer: &TextDrawer) -> Node<()> {
        Node::Layout(
            "bar".into(),
            Style {
                display: Display::Grid,
                grid_template_rows: vec![percent(1.0)],
                grid_template_columns: (0..SIZE).map(|_| percent(0.166)).collect(),
                size: Size {
                    // width: percent(1.0),
                    width: auto(),
                    height: percent(1.0),
                    // width: length(400.0),
                    // height: length(100.0),
                },
                aspect_ratio: Some(SIZE as f32),
                ..Default::default()
            },
            self.values
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    Node::Anonym(
                        Style {
                            size: Size {
                                width: percent(1.0),
                                height: percent(1.0),
                            },
                            grid_row: line(1),
                            grid_column: line(1 + i as i16),
                            ..Default::default()
                        },
                        vec![Node::Layout(
                            v.id.clone(),
                            Style {
                                size: Size {
                                    width: percent(1.0),
                                    height: percent(1.0),
                                },
                                ..Default::default()
                            },
                            vec![v.item.layout(text_drawer)],
                        )],
                    )
                })
                .collect(),
        )
    }
    fn update(&mut self, _rects: &TaffyRectNode<()>) -> UpdateResult {
        UpdateResult::Continue
    }
}

// "style": {
//     "size": "100% 100%",
//     "display": "Grid",
//     "grid_template_rows": ["100%"],
//     "grid_template_columns": ["16.66%", "16.66%", "16.66%", "16.66%", "16.66%", "16.66%"],
//   },
// "children": [
//     {
//         "style": {
//         "grid_row": ["line 1", "line 1"],
//         "grid_column": ["line 1", "line 1"],
//         },
//         "children": [
//            {
