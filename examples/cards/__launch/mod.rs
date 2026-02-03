mod board_launch;
mod value_bar;
pub use board_launch::*;
pub use value_bar::*;
pub trait Launch {
    fn update(&mut self);
}

pub enum LauchType {
    Board,
    Label,
    Value,
    ValueBar,
    Card,
}
