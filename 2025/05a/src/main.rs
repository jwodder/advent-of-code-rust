use adventutil::Input;
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Eq, PartialEq)]
struct InclusiveRangeSet(Vec<RangeInclusive<u64>>);

impl InclusiveRangeSet {
    fn new() -> InclusiveRangeSet {
        InclusiveRangeSet(Vec::new())
    }

    fn insert(&mut self, new_r: RangeInclusive<u64>) {
        let (new_start, new_end) = new_r.into_inner();
        match self.0.binary_search_by(|r| {
            if r.end() < &new_start {
                Ordering::Less
            } else if &new_end < r.start() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) {
            Ok(i) => {
                let mut idx_start = i;
                let mut idx_end = i;
                let (mut start, mut end) = self.0[i].clone().into_inner();
                if new_start < start {
                    start = new_start;
                    while let Some(j) = idx_start.checked_sub(1)
                        && &start <= self.0[j].end()
                    {
                        idx_start = j;
                        start = start.min(*self.0[j].start());
                    }
                }
                if end < new_end {
                    end = new_end;
                    while let Some(j) = idx_end.checked_add(1)
                        && j < self.0.len()
                        && self.0[j].start() < &end
                    {
                        idx_end = j;
                        end = end.max(*self.0[j].end());
                    }
                }
                self.0.drain(idx_start..=idx_end);
                self.0.insert(idx_start, start..=end);
            }
            Err(i) => self.0.insert(i, new_start..=new_end),
        }
    }

    fn contains(&self, value: u64) -> bool {
        self.0
            .binary_search_by(|r| {
                if *r.end() < value {
                    Ordering::Less
                } else if value < *r.start() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
    }
}

fn solve(input: Input) -> usize {
    let (fresh_text, available) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let mut fresh = InclusiveRangeSet::new();
    for ln in fresh_text.lines() {
        let (start, end) = ln.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        fresh.insert(start..=end);
    }
    let mut qty = 0;
    for ln in available.lines() {
        let id = ln.parse::<u64>().unwrap();
        if fresh.contains(id) {
            qty += 1;
        }
    }
    qty
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n");
        assert_eq!(solve(input), 3);
    }
}
