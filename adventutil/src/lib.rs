pub mod closure;
pub mod counter;
pub mod grid;
pub mod gridgeom;
pub mod index;
pub mod intcode;
pub mod maxn;
#[cfg(feature = "ocr")]
pub mod ocr;
pub mod pullparser;
use num_traits::{Bounded, PrimInt};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{self, read_to_string, stdin, BufRead, BufReader};
use std::iter::FusedIterator;
use std::ops::{Bound, RangeBounds};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Stdin,
    File(PathBuf),
    Str(&'static str),
}

impl Input {
    pub fn from_env() -> Input {
        std::env::args_os()
            .nth(1)
            .map(|s| Input::File(s.into()))
            .unwrap_or(Input::Stdin)
    }

    pub fn read(self) -> String {
        match self {
            Input::Stdin => read_to_string(stdin().lock()).expect("Error reading stdin"),
            Input::File(path) => fs::read_to_string(path).expect("Error reading file"),
            Input::Str(s) => s.to_string(),
        }
    }

    pub fn lines(self) -> Lines {
        match self {
            Input::Stdin => Lines::Stdin(stdin().lines()),
            Input::File(path) => {
                Lines::File(BufReader::new(File::open(path).expect("Error opening file")).lines())
            }
            Input::Str(s) => Lines::Str(s.lines()),
        }
    }

    // Yields each paragraph with inner newlines converted to '\n' and trailing
    // newlines removed
    pub fn paragraphs(self) -> Paragraphs {
        Paragraphs::new(self.lines())
    }

    pub fn parse<T: FromStr>(self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        match self {
            Input::Str(s) => s.trim().parse::<T>(),
            input => input.read().trim().parse::<T>(),
        }
        .expect("Error parsing input")
    }

    pub fn parse_lines<T: FromStr>(self) -> impl Iterator<Item = T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.lines()
            .map(|s| s.parse::<T>().expect("Error parsing input"))
    }

    // Assumes that the input is just one line of comma-separated values
    pub fn parse_csv_line<T: FromStr>(self) -> Vec<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        parse_csv(&self.read())
    }
}

impl From<&'static str> for Input {
    fn from(s: &'static str) -> Input {
        Input::Str(s)
    }
}

pub enum Lines {
    Stdin(io::Lines<io::StdinLock<'static>>),
    File(io::Lines<BufReader<File>>),
    Str(std::str::Lines<'static>),
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self {
            Lines::Stdin(i) => i.next().map(|r| r.expect("Error reading input")),
            Lines::File(i) => i.next().map(|r| r.expect("Error reading input")),
            Lines::Str(i) => i.next().map(|s| s.to_string()),
        }
    }
}

pub struct Paragraphs {
    inner: Lines,
    buffer: Vec<String>,
}

impl Paragraphs {
    fn new(inner: Lines) -> Paragraphs {
        Paragraphs {
            inner,
            buffer: Vec::new(),
        }
    }
}

impl Iterator for Paragraphs {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        for ln in self.inner.by_ref() {
            if ln.is_empty() {
                if !self.buffer.is_empty() {
                    let s = self.buffer.join("\n");
                    self.buffer.clear();
                    return Some(s);
                }
            } else {
                self.buffer.push(ln);
            }
        }
        if !self.buffer.is_empty() {
            let s = self.buffer.join("\n");
            self.buffer.clear();
            return Some(s);
        }
        None
    }
}

pub fn parse_csv<T: FromStr>(s: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.trim()
        .split(',')
        .map(|t| t.parse::<T>().expect("Error parsing input"))
        .collect()
}

/// Returns an iterator of all pairs `(i, j)` where `0 <= i < j < size`, i.e.,
/// an iterator of all increasing pairs of indices into a slice of length
/// `size`.
pub fn unordered_index_pairs(size: usize) -> UnorderedIndexPairs {
    UnorderedIndexPairs::new(size)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnorderedIndexPairs {
    size: usize,
    i: usize,
    j: usize,
}

impl UnorderedIndexPairs {
    fn new(size: usize) -> Self {
        UnorderedIndexPairs { size, i: 0, j: 1 }
    }
}

impl Iterator for UnorderedIndexPairs {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.i + 1 >= self.size {
            None
        } else {
            let r = (self.i, self.j);
            self.j += 1;
            if self.j >= self.size {
                self.i += 1;
                self.j = self.i + 1;
            }
            Some(r)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = if self.i + 1 >= self.size {
            0
        } else {
            fn sum_up_to(n: usize) -> usize {
                if n == 0 {
                    0
                } else {
                    (n * (n - 1)) / 2
                }
            }

            sum_up_to(self.size - self.i) - (self.j - (self.i + 1))
        };
        (sz, Some(sz))
    }
}

impl FusedIterator for UnorderedIndexPairs {}

impl ExactSizeIterator for UnorderedIndexPairs {}

/// Returns an iterator of all pairs `(&a, &b)` of elements of `array` where
/// the index of `a` is less than the index of `b`.
pub fn unordered_pairs<T>(array: &[T]) -> UnorderedPairs<'_, T> {
    UnorderedPairs::new(array)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnorderedPairs<'a, T> {
    array: &'a [T],
    inner: UnorderedIndexPairs,
}

impl<'a, T> UnorderedPairs<'a, T> {
    fn new(array: &'a [T]) -> Self {
        UnorderedPairs {
            array,
            inner: unordered_index_pairs(array.len()),
        }
    }
}

impl<'a, T> Iterator for UnorderedPairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j) = self.inner.next()?;
        Some((&self.array[i], &self.array[j]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T> FusedIterator for UnorderedPairs<'a, T> {}

impl<'a, T> ExactSizeIterator for UnorderedPairs<'a, T> {}

/// Returns the length of the shortest path from `start` to `end`.  `func` must
/// be a function that takes a vertex `v` and returns an iterable of all of its
/// neighbors and their distances from `v`.  Returns `None` if there is no
/// route to `end`.
///
/// `func` will not be called with `&end` as an argument.
pub fn dijkstra_length<T, F, I>(start: T, end: T, mut func: F) -> Option<u32>
where
    T: Eq + Hash + Clone,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = (T, u32)>,
{
    let mut visited = HashSet::new();
    let mut distances = HashMap::from([(start, 0)]);
    loop {
        let (current, dist) = distances
            .iter()
            .filter(|&(k, _)| !visited.contains(k))
            .min_by_key(|&(_, &dist)| dist)
            .map(|(k, &dist)| (k.clone(), dist))?;
        if current == end {
            return Some(dist);
        }
        for (p, d) in func(&current) {
            if !visited.contains(&p) {
                let newdist = dist + d;
                match distances.entry(p) {
                    Entry::Vacant(e) => {
                        e.insert(newdist);
                    }
                    Entry::Occupied(mut e) if *e.get() > newdist => {
                        e.insert(newdist);
                    }
                    _ => (),
                }
            }
        }
        visited.insert(current);
    }
}

pub trait FromBits: PrimInt {
    /// Converts an iterable of bits (most significant first) into an integer.
    /// Does not guard against overflow/underflow.
    ///
    /// # Example
    ///
    /// ```
    /// use adventutil::FromBits;
    ///
    /// let n = u32::from_bits([true, false, true, false, true, false]);
    /// assert_eq!(n, 42);
    /// ```
    // TODO: Return None or Err on overflow?
    fn from_bits<I: IntoIterator<Item = bool>>(bits: I) -> Self {
        bits.into_iter().fold(Self::zero(), |n, b| {
            (n << 1) + if b { Self::one() } else { Self::zero() }
        })
    }
}

impl<T: PrimInt> FromBits for T {}

pub fn ranges_overlap<T, R1, R2>(range1: R1, range2: R2) -> bool
where
    T: PrimInt,
    R1: RangeBounds<T>,
    R2: RangeBounds<T>,
{
    let min1 = match range1.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => {
            let Some(x1) = x.checked_add(&T::one()) else {
                return false;
            };
            x1
        }
        Bound::Unbounded => Bounded::min_value(),
    };
    let min2 = match range2.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => {
            let Some(x1) = x.checked_add(&T::one()) else {
                return false;
            };
            x1
        }
        Bound::Unbounded => Bounded::min_value(),
    };
    let minimum = min1.max(min2);
    // The following variables are inclusive maximums:
    let max1 = match range1.end_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => {
            let Some(x1) = x.checked_sub(&T::one()) else {
                return false;
            };
            x1
        }
        Bound::Unbounded => Bounded::max_value(),
    };
    let max2 = match range2.end_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => {
            let Some(x1) = x.checked_sub(&T::one()) else {
                return false;
            };
            x1
        }
        Bound::Unbounded => Bounded::max_value(),
    };
    let maximum = max1.min(max2);
    minimum <= maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unordered_index_pairs() {
        let mut iter = unordered_index_pairs(4);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((0, 2)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((0, 3)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((1, 3)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_index_pairs_size_2() {
        let mut iter = unordered_index_pairs(2);
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_index_pairs_size_1() {
        let mut iter = unordered_index_pairs(1);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_index_pairs_size_0() {
        let mut iter = unordered_index_pairs(0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_pairs() {
        let mut iter = unordered_pairs(&[1, 2, 3, 4]);
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.size_hint(), (5, Some(5)));
        assert_eq!(iter.next(), Some((&1, &3)));
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some((&2, &4)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((&3, &4)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_pairs_size_2() {
        let mut iter = unordered_pairs(&[1, 2]);
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_pairs_size_1() {
        let mut iter = unordered_pairs(&[1]);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_unordered_pairs_size_0() {
        let mut iter: UnorderedPairs<'_, i32> = unordered_pairs(&[]);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_ranges_overlap() {
        assert!(ranges_overlap(0..5, 0..3));
        assert!(ranges_overlap(0..3, 0..5));
        assert!(ranges_overlap(3..7, 0..10));
        assert!(!ranges_overlap(0..3, 3..5));
        assert!(ranges_overlap(0..=3, 3..5));
        assert!(ranges_overlap::<usize, _, _>(.., ..));
        assert!(!ranges_overlap(0..5, 3..0));
        assert!(!ranges_overlap(
            (Bound::Excluded(usize::MAX), Bound::Unbounded),
            0..5
        ));
        assert!(!ranges_overlap(
            (Bound::Unbounded, Bound::Excluded(usize::MIN)),
            0..5
        ));
        assert!(ranges_overlap(
            (Bound::Excluded(usize::MIN), Bound::Unbounded),
            0..5
        ));
        assert!(ranges_overlap(
            (Bound::Unbounded, Bound::Excluded(usize::MAX)),
            0..5
        ));
        assert!(ranges_overlap(.., 0..5));
    }
}
