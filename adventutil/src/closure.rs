use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

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
