#[derive(Clone, Debug)]
pub struct TaffyChild<T> {
    pub id: String,
    pub item: T,
}

impl<T> TaffyChild<T> {
    pub fn new(id: &str, item: T) -> Self {
        Self {
            id: id.to_string(),
            item,
        }
    }
}
