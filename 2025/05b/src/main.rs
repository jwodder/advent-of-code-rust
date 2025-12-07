use adventutil::Input;
use adventutil::ranges::parse_range;
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
}

fn solve(input: Input) -> u64 {
    let fresh_text = input
        .paragraphs()
        .next()
        .expect("input should not be empty");
    let mut fresh = InclusiveRangeSet::new();
    for ln in fresh_text.lines() {
        fresh.insert(parse_range::<u64>(ln).unwrap());
    }
    fresh.0.into_iter().map(|r| *r.end() - *r.start() + 1).sum()
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
        assert_eq!(solve(input), 14);
    }
}
