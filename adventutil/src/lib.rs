pub mod area;
pub mod counter;
pub mod grid;
pub mod gridgeom;
pub mod index;
pub mod intcode;
pub mod maxn;
pub mod maxtracker;
pub mod numtheory;
#[cfg(feature = "ocr")]
pub mod ocr;
pub mod pullparser;
pub mod ranges;
use num_traits::PrimInt;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque, hash_map::Entry};
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, read_to_string, stdin};
use std::path::PathBuf;
use std::rc::Rc;
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
            .map_or(Input::Stdin, |s| Input::File(s.into()))
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

#[derive(Debug)]
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
            Lines::Str(i) => i.next().map(ToString::to_string),
        }
    }
}

#[derive(Debug)]
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
                if n == 0 { 0 } else { (n * (n - 1)) / 2 }
            }

            sum_up_to(self.size - self.i) - (self.j - (self.i + 1))
        };
        (sz, Some(sz))
    }
}

impl std::iter::FusedIterator for UnorderedIndexPairs {}

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

impl<T> std::iter::FusedIterator for UnorderedPairs<'_, T> {}

impl<T> ExactSizeIterator for UnorderedPairs<'_, T> {}

/// Returns the length of the shortest path from `start` to a node that
/// satisfies `is_end`.  `func` must be a function that takes a vertex `v` and
/// returns an iterable of all of its neighbors (which must not include `v`
/// itself) and their distances from `v`.  Returns `None` if there is no such
/// path.
///
/// `func` will not be called with the end node as an argument.
pub fn dijkstra_length<T, P, F, I>(start: T, is_end: P, mut func: F) -> Option<u32>
where
    T: Eq + Hash,
    P: Fn(&T) -> bool,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = (T, u32)>,
{
    let mut visited = HashSet::new();
    let mut distances = DistanceMap::from([(start, 0)]);
    loop {
        let (current, dist) = distances.pop_nearest()?;
        if is_end(&current) {
            return Some(dist);
        }
        for (p, d) in func(&current) {
            if !visited.contains(&p) {
                distances.insert(p, dist + d);
            }
        }
        visited.insert(current);
    }
}

#[derive(Clone, Debug)]
pub struct DistanceMap<T> {
    node2dist: HashMap<Rc<T>, u32>,
    dist2nodes: BTreeMap<u32, HashSet<Rc<T>>>,
}

impl<T> DistanceMap<T> {
    pub fn new() -> DistanceMap<T> {
        DistanceMap {
            node2dist: HashMap::new(),
            dist2nodes: BTreeMap::new(),
        }
    }
}

impl<T: Eq + Hash> DistanceMap<T> {
    pub fn insert(&mut self, node: T, distance: u32) {
        let node = Rc::new(node);
        match self.node2dist.entry(Rc::clone(&node)) {
            Entry::Vacant(e) => {
                e.insert(distance);
                self.dist2nodes.entry(distance).or_default().insert(node);
            }
            Entry::Occupied(mut e) if *e.get() > distance => {
                let old_dist = *e.get();
                e.insert(distance);
                if let Some(old_set) = self.dist2nodes.get_mut(&old_dist) {
                    old_set.remove(&node);
                }
                self.dist2nodes.entry(distance).or_default().insert(node);
            }
            _ => (),
        }
    }

    pub fn pop_nearest(&mut self) -> Option<(T, u32)> {
        loop {
            let mut e = self.dist2nodes.first_entry()?;
            let distance = *e.key();
            let mut first = true;
            if let Some(node) = e
                .get_mut()
                .extract_if(|_| std::mem::replace(&mut first, false))
                .next()
            {
                self.node2dist.remove(&node);
                return Some((
                    Rc::into_inner(node).expect("Rc<node> should have exactly one reference"),
                    distance,
                ));
            } else {
                // This HashSet is empty; go to the next one.
                e.remove();
            }
        }
    }
}

impl<T> Default for DistanceMap<T> {
    fn default() -> DistanceMap<T> {
        DistanceMap::new()
    }
}

impl<T: Eq + Hash, const N: usize> From<[(T, u32); N]> for DistanceMap<T> {
    fn from(data: [(T, u32); N]) -> DistanceMap<T> {
        let mut map = DistanceMap::new();
        for (node, dist) in data {
            map.insert(node, dist);
        }
        map
    }
}

impl<T: Eq + Hash> Extend<(T, u32)> for DistanceMap<T> {
    fn extend<I: IntoIterator<Item = (T, u32)>>(&mut self, iter: I) {
        for (node, dist) in iter {
            self.insert(node, dist);
        }
    }
}

impl<T: Eq + Hash> FromIterator<(T, u32)> for DistanceMap<T> {
    fn from_iter<I: IntoIterator<Item = (T, u32)>>(iter: I) -> DistanceMap<T> {
        let mut map = DistanceMap::new();
        for (node, dist) in iter {
            map.insert(node, dist);
        }
        map
    }
}

/// Returns the length of the shortest path from `start` to a node that
/// satisfies `is_end`.  `func` must be a function that takes a vertex `v` and
/// returns an iterable of all of its neighbors, which are assumed to each be
/// one unit of distance away (and must not include `v` itself).  Returns
/// `None` if there is no such path.
///
/// `func` will not be called with the end node as an argument.
pub fn unit_dijkstra_length<T, P, F, I>(start: T, is_end: P, mut func: F) -> Option<u32>
where
    T: Eq + Hash + Clone + std::fmt::Debug,
    P: Fn(&T) -> bool,
    F: FnMut(&T) -> I,
    I: IntoIterator<Item = T>,
{
    let mut seen = HashSet::new();
    let mut dist = 0;
    let mut states = vec![start];
    while !states.is_empty() {
        let mut states2 = Vec::new();
        for n in states {
            if is_end(&n) {
                return Some(dist);
            }
            for n2 in func(&n) {
                if seen.insert(n2.clone()) {
                    states2.push(n2);
                }
            }
        }
        states = states2;
        dist += 1;
    }
    None
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

pub fn one2many_closure<T, F, I>(seed: T, mut generator: F) -> HashSet<T>
where
    T: Eq + Hash + Clone,
    F: FnMut(T) -> I,
    I: IntoIterator<Item = T>,
{
    let mut seen = HashSet::from([seed.clone()]);
    let mut queue = VecDeque::from([seed]);
    while let Some(value) = queue.pop_front() {
        for new in generator(value) {
            if seen.insert(new.clone()) {
                queue.push_back(new);
            }
        }
    }
    seen
}

/// Given an undirected, unweighted, simple graph with vertices `vertices` and
/// function `adjacent` mapping vertices to their neighbors, compute the
/// graph's connected components.
pub fn components<T, I, F, J>(vertices: I, adjacent: F) -> Vec<HashSet<T>>
where
    T: Eq + Hash + Clone,
    I: IntoIterator<Item = T>,
    F: FnMut(T) -> J + Clone,
    J: IntoIterator<Item = T>,
{
    let mut comps = Vec::new();
    let mut comped = HashSet::new();
    for v in vertices {
        if comped.insert(v.clone()) {
            let c = one2many_closure(v, adjacent.clone());
            comped.extend(c.iter().cloned());
            comps.push(c);
        }
    }
    comps
}

/// Given a pure function `advance()` that ultimately enters a cycle,
/// `cyclic_nth(state, advance, n)` determines the result of applying
/// `advance()` to `state` `n` times.
pub fn cyclic_nth<T, F>(mut state: T, mut advance: F, n: u64) -> T
where
    T: Eq + Hash,
    F: FnMut(&T) -> T,
{
    let mut seen = HashMap::new();
    for i in 0..n {
        let newstate = advance(&state);
        match seen.entry(state) {
            Entry::Occupied(e) => {
                let &j = e.get();
                let k = (n - j) % (i - j) + j;
                return seen
                    .into_iter()
                    .find_map(|(st, x)| (x == k).then_some(st))
                    .unwrap();
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
        state = newstate;
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unordered_index_pairs {
        use super::*;

        #[test]
        fn size4() {
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
        fn size2() {
            let mut iter = unordered_index_pairs(2);
            assert_eq!(iter.size_hint(), (1, Some(1)));
            assert_eq!(iter.next(), Some((0, 1)));
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn size1() {
            let mut iter = unordered_index_pairs(1);
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn size0() {
            let mut iter = unordered_index_pairs(0);
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
    }

    mod unordered_pairs {
        use super::*;

        #[test]
        fn size4() {
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
        fn size2() {
            let mut iter = unordered_pairs(&[1, 2]);
            assert_eq!(iter.size_hint(), (1, Some(1)));
            assert_eq!(iter.next(), Some((&1, &2)));
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn size1() {
            let mut iter = unordered_pairs(&[1]);
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn size0() {
            let mut iter: UnorderedPairs<'_, i32> = unordered_pairs(&[]);
            assert_eq!(iter.size_hint(), (0, Some(0)));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next(), None);
        }
    }
}
