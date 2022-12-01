use std::cmp::Reverse;

pub struct MaxN<T> {
    n: usize,
    values: Vec<T>,
}

impl<T> MaxN<T> {
    pub fn new(n: usize) -> Self {
        MaxN {
            n,
            values: Vec::with_capacity(n + 1),
        }
    }

    pub fn add(&mut self, value: T)
    where
        T: Ord,
    {
        match self.values.binary_search_by_key(&Reverse(&value), Reverse) {
            Err(i) if i == self.values.len() && self.values.len() == self.n => (),
            Ok(i) | Err(i) => {
                self.values.insert(i, value);
                self.values.truncate(self.n);
            }
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

impl<T> IntoIterator for MaxN<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<T: Ord> Extend<T> for MaxN<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.add(value);
        }
    }
}

pub fn maxn<T, I>(n: usize, iter: I) -> Vec<T>
where
    T: Ord,
    I: IntoIterator<Item = T>,
{
    let mut maxer = MaxN::new(n);
    maxer.extend(iter);
    maxer.into_vec()
}
