use std::borrow::Borrow;
use std::collections::{
    HashMap,
    hash_map::{IntoIter, IntoValues},
};
use std::hash::Hash;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Counter<T> {
    inner: HashMap<T, u64>,
}

impl<T: Eq + Hash> PartialEq for Counter<T> {
    fn eq(&self, other: &Counter<T>) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq + Hash> Eq for Counter<T> {}

impl<T> Counter<T> {
    pub fn new() -> Counter<T> {
        Counter {
            inner: HashMap::new(),
        }
    }

    pub fn into_values(self) -> IntoValues<T, u64> {
        self.inner.into_values()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: Eq + Hash> Counter<T> {
    pub fn add(&mut self, value: T) {
        *self.inner.entry(value).or_insert(0) += 1;
    }

    pub fn add_qty(&mut self, value: T, qty: u64) {
        *self.inner.entry(value).or_insert(0) += qty;
    }

    pub fn total(&self) -> u64 {
        self.inner.values().copied().sum()
    }
}

impl<T> Default for Counter<T> {
    fn default() -> Counter<T> {
        Counter::new()
    }
}

impl<T: Eq + Hash> Extend<T> for Counter<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.add(x);
        }
    }
}

impl<T: Eq + Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Counter<T> {
        let mut counter = Counter::new();
        for x in iter {
            counter.add(x);
        }
        counter
    }
}

impl<T, U> Index<&U> for Counter<T>
where
    T: Eq + Hash + Borrow<U>,
    U: Eq + Hash + ?Sized,
{
    type Output = u64;

    fn index(&self, key: &U) -> &u64 {
        self.inner.get(key).unwrap_or(&0)
    }
}

impl<T> IntoIterator for Counter<T> {
    type Item = (T, u64);
    type IntoIter = IntoIter<T, u64>;

    fn into_iter(self) -> IntoIter<T, u64> {
        self.inner.into_iter()
    }
}
