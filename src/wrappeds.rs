#[derive(Clone, Debug)]
pub struct WrappedId<T> {
    pub id: String,
    pub item: T,
}

impl<T> WrappedId<T> {
    pub fn new(id: &str, item: T) -> Self {
        Self {
            id: id.to_string(),
            item,
        }
    }
}
#[derive(Clone, Debug)]
pub struct WrappedIds<T> {
    pub items: Vec<WrappedId<T>>,
}

impl<T> WrappedIds<T> {
    pub fn new(name: &str, items: impl Iterator<Item = T>) -> Self {
        Self {
            items: items
                .into_iter()
                .enumerate()
                .map(|(i, c)| WrappedId::new(&format!("{}_{}", name, i), c))
                .collect(),
        }
    }
    pub fn anonym(items: impl Iterator<Item = T>) -> Self {
        Self {
            items: items
                .enumerate()
                .map(|(i, c)| WrappedId::new(&format!("{}", i), c))
                .collect(),
        }
    }
}
