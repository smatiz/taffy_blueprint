use crate::components::*;

pub enum LauchType {
    Board,
    Label,
    Value,
    ValueBar,
    ValueBarItem,
    Card,
}

pub struct Launch {
    component: Box<dyn Component>,
    text_drawer: TextDrawer,
}

impl Launch {
    pub async fn new(text_drawer: TextDrawer, component: Box<dyn Component>) -> Self {
        Self {
            text_drawer,
            component,
        }
    }

    pub fn update(&mut self) {
        let rects = crate::components::helper_macroquad::wrapped_into_screen_root_node(
            self.component.layout(&self.text_drawer),
        );
        self.component.update(&rects);
        self.component.draw(&self.text_drawer, &rects);
    }
}
