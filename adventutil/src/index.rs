use std::collections::{
    hash_map::{IntoIter, IntoKeys, IntoValues},
    HashMap,
};
use std::hash::Hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Index<T: Eq + Hash>(HashMap<T, usize>);

impl<T: Eq + Hash> Index<T> {
    pub fn new() -> Index<T> {
        Index(HashMap::new())
    }

    pub fn get(&mut self, value: T) -> usize {
        let i = self.0.len();
        *(self.0.entry(value).or_insert(i))
    }

    pub fn insert(&mut self, value: T) {
        let _ = self.get(value);
    }

    pub fn get_by_index(&self, i: usize) -> Option<&T> {
        self.0
            .iter()
            .find_map(|(value, &j)| (i == j).then_some(value))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_keys(self) -> IntoKeys<T, usize> {
        self.0.into_keys()
    }

    pub fn into_indices(self) -> IntoValues<T, usize> {
        self.0.into_values()
    }
}

impl<T: Eq + Hash> Default for Index<T> {
    fn default() -> Index<T> {
        Index::new()
    }
}

impl<T: Eq + Hash> IntoIterator for Index<T> {
    type Item = (T, usize);
    type IntoIter = IntoIter<T, usize>;

    fn into_iter(self) -> IntoIter<T, usize> {
        self.0.into_iter()
    }
}

impl<T: Eq + Hash> FromIterator<T> for Index<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Index<T> {
        let mut index = Index::new();
        for value in iter {
            index.insert(value);
        }
        index
    }
}
