#[derive(Clone, Debug)]
pub struct ComponentId<T> {
    pub id: String,
    pub item: T,
}

impl<T> ComponentId<T> {
    pub fn new(id: &str, item: T) -> Self {
        Self {
            id: id.to_string(),
            item,
        }
    }
}
