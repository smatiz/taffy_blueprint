use crate::components::Character;

pub enum UpdateResult {
    Continue,
    End(Character),
}
