pub enum UpdateResult<T> {
    Continue,
    End(T),
}
impl<T> UpdateResult<T> {
    pub fn finished(&self) -> bool {
        match self {
            UpdateResult::Continue => false,
            UpdateResult::End(_) => true,
        }
    }

    pub fn if_all_end(results: &[Self]) -> UpdateResult<()> {
        // let mut v = vec![];
        for result in results {
            match result {
                UpdateResult::Continue => return UpdateResult::Continue,
                UpdateResult::End(_) => {
                    // v.push(value);
                }
            }
        }
        return UpdateResult::End(());
    }
}
