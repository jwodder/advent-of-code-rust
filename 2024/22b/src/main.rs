use adventutil::Input;
use std::collections::HashMap;

type Int = i64;
type Change = (Int, Int, Int, Int);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Buyer(Int);

impl Buyer {
    fn rand(&mut self) -> Int {
        let mut secret = self.0;
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;
        self.0 = secret;
        secret % 10
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ChangeTracker {
    last_change: Change,
    last_price: Int,
}

impl ChangeTracker {
    fn new(a: Int, b: Int, c: Int, d: Int, e: Int) -> ChangeTracker {
        let last_change = ((b - a), (c - b), (d - c), (e - d));
        let last_price = e;
        ChangeTracker {
            last_change,
            last_price,
        }
    }

    fn add(&mut self, price: Int) -> Change {
        let ch = price - self.last_price;
        let (_, b, c, d) = self.last_change;
        self.last_change = (b, c, d, ch);
        self.last_price = price;
        self.last_change
    }
}

fn solve(input: Input) -> Int {
    let mut changes2bananas = HashMap::<Change, Int>::new();
    for i in input.parse_lines::<Int>() {
        let mut buyer = Buyer(i);
        let mut first_changes = HashMap::new();
        let a = buyer.rand();
        let b = buyer.rand();
        let c = buyer.rand();
        let d = buyer.rand();
        let e = buyer.rand();
        let mut tracker = ChangeTracker::new(a, b, c, d, e);
        first_changes.insert(tracker.last_change, e);
        for _ in 0..1995 {
            let price = buyer.rand();
            let change = tracker.add(price);
            first_changes.entry(change).or_insert(price);
        }
        for (change, price) in first_changes {
            *changes2bananas.entry(change).or_default() += price;
        }
    }
    changes2bananas.into_values().max().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("1\n2\n3\n2024\n");
        assert_eq!(solve(input), 23);
    }
}
