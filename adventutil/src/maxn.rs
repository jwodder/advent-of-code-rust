use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
pub struct MaxN<T> {
    n: usize,
    heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> MaxN<T> {
    pub fn new(n: usize) -> Self {
        MaxN {
            n,
            heap: BinaryHeap::with_capacity(n + 1),
        }
    }

    pub fn add(&mut self, value: T) {
        self.heap.push(Reverse(value));
        if self.heap.len() > self.n {
            let _ = self.heap.pop();
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(v)| v)
            .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maxn() {
        assert_eq!(maxn(5, [1, 10, 5, 42, 3, 0, 23, 17]), [42, 23, 17, 10, 5]);
    }

    #[test]
    fn test_maxn_less_than_n() {
        assert_eq!(maxn(5, [1, 10, 5]), [10, 5, 1]);
    }

    #[test]
    fn test_maxn_empty() {
        assert_eq!(maxn::<u32, _>(5, []), []);
    }
}
