use std::collections::HashMap;
use std::hash::Hash;

pub struct Index<T>(HashMap<T, usize>);

impl<T> Index<T> {
    pub fn new() -> Index<T> {
        Index(HashMap::new())
    }

    pub fn get(&mut self, value: T) -> usize
    where
        T: Eq + Hash,
    {
        let i = self.0.len();
        *(self.0.entry(value).or_insert(i))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Default for Index<T> {
    fn default() -> Index<T> {
        Index::new()
    }
}
