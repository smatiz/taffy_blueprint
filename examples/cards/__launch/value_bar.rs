use crate::components::*;

pub struct ValueBarLaunch {
    component: ValueBarComponent,
    text_drawer: TextDrawer,
}

impl ValueBarLaunch {
    pub async fn new() -> Self {
        let text_drawer = TextDrawer::new(16).await;
        let component = ValueBarComponent::new(3);

        Self {
            text_drawer,
            component,
        }
    }

    pub fn update(&mut self) {
        let rects = crate::components::helper_macroquad::wrapped_into_screen_root_node(
            self.component.layout(&self.text_drawer),
        );
        self.component.draw(&self.text_drawer, &rects);
    }
}
