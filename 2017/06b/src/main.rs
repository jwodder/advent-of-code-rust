use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Banks(Vec<u32>);

impl Banks {
    fn maxpos(&self) -> usize {
        self.0
            .iter()
            .copied()
            .enumerate()
            // Use rev() so that max_by_key() will return the first max element
            // in self.0 when there's a tie.
            .rev()
            .max_by_key(|&(_, val)| val)
            .unwrap()
            .0
    }

    fn redistribute(&mut self) {
        let mut i = self.maxpos();
        let blocks = std::mem::take(&mut self.0[i]);
        for _ in 0..blocks {
            i = (i + 1) % self.0.len();
            self.0[i] += 1;
        }
    }
}

impl std::str::FromStr for Banks {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Banks, ParseError> {
        PullParser::new(s)
            .delimited(Token::Whitespace, |t| t.parse::<u32>().map_err(Into::into))
            .map(Banks)
    }
}

fn solve(input: Input) -> usize {
    let mut last_seen = std::collections::HashMap::new();
    let mut banks = input.parse::<Banks>();
    let mut cycles = 0;
    last_seen.insert(banks.clone(), cycles);
    loop {
        banks.redistribute();
        cycles += 1;
        if let Some(i) = last_seen.insert(banks.clone(), cycles) {
            return cycles - i;
        }
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_example() {
        let input = Input::from("0 2 7 0");
        assert_eq!(solve(input), 4);
    }

    #[rstest]
    #[case("0 2 7 0", "2 4 1 2")]
    #[case("2 4 1 2", "3 1 2 3")]
    #[case("3 1 2 3", "0 2 3 4")]
    #[case("0 2 3 4", "1 3 4 1")]
    #[case("1 3 4 1", "2 4 1 2")]
    fn test_redistribute(#[case] mut before: Banks, #[case] after: Banks) {
        before.redistribute();
        assert_eq!(before, after);
    }
}
