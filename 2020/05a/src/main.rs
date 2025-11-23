use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BinarySplitter {
    low: u32,
    high: u32,
}

impl BinarySplitter {
    fn new(size: u32) -> Self {
        BinarySplitter { low: 0, high: size }
    }

    fn lower(&mut self) {
        self.high = self.high.midpoint(self.low);
    }

    fn upper(&mut self) {
        self.low = self.high.midpoint(self.low);
    }

    fn get(&self) -> Option<u32> {
        (self.low + 1 == self.high).then_some(self.low)
    }
}

fn pass2id(pass: &str) -> u32 {
    let mut chars = pass.chars();
    let mut row_search = BinarySplitter::new(128);
    for _ in 0..7 {
        match chars.next() {
            Some('F') => row_search.lower(),
            Some('B') => row_search.upper(),
            Some(c) => panic!("Invalid row specifier: {c:?}"),
            None => panic!("Short boarding pass"),
        }
    }
    let row = row_search.get().unwrap();
    let mut column_search = BinarySplitter::new(8);
    for _ in 0..3 {
        match chars.next() {
            Some('L') => column_search.lower(),
            Some('R') => column_search.upper(),
            Some(c) => panic!("Invalid column specifier: {c:?}"),
            None => panic!("Short boarding pass"),
        }
    }
    let column = column_search.get().unwrap();
    row * 8 + column
}

fn solve(input: Input) -> u32 {
    input.lines().map(|s| pass2id(&s)).max().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("FBFBBFFRLR", 357)]
    #[case("BFFFBBFRRR", 567)]
    #[case("FFFBBBFRRR", 119)]
    #[case("BBFFBBFRLL", 820)]
    fn examples(#[case] pass: &str, #[case] id: u32) {
        assert_eq!(pass2id(pass), id);
    }
}
