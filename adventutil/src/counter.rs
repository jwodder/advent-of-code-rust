use std::borrow::Borrow;
use std::collections::{
    hash_map::{IntoIter, IntoValues},
    HashMap,
};
use std::hash::Hash;
use std::iter::{Extend, FromIterator};
use std::ops::Index;

pub struct Counter<T> {
    inner: HashMap<T, usize>,
}

impl<T> Counter<T> {
    pub fn new() -> Counter<T> {
        Counter {
            inner: HashMap::new(),
        }
    }

    pub fn into_values(self) -> IntoValues<T, usize> {
        self.inner.into_values()
    }
}

impl<T: Eq + Hash> Counter<T> {
    pub fn add(&mut self, value: T) {
        *self.inner.entry(value).or_insert(0) += 1;
    }

    pub fn add_qty(&mut self, value: T, qty: usize) {
        *self.inner.entry(value).or_insert(0) += qty;
    }

    pub fn total(&self) -> usize {
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

impl<T, U: ?Sized> Index<&U> for Counter<T>
where
    T: Eq + Hash + Borrow<U>,
    U: Eq + Hash,
{
    type Output = usize;

    fn index(&self, key: &U) -> &usize {
        self.inner.get(key).unwrap_or(&0)
    }
}

impl<T> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = IntoIter<T, usize>;

    fn into_iter(self) -> IntoIter<T, usize> {
        self.inner.into_iter()
    }
}
