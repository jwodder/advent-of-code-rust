use adventutil::Input;
use itertools::{Itertools, MinMaxResult};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Digit {
    pos: usize,
    value: u32,
}

fn line2number(s: &str) -> u32 {
    let r = s
        .char_indices()
        .filter(|&(_, ch)| ch.is_ascii_digit())
        .map(|(pos, ch)| {
            let value = ch.to_digit(10).unwrap();
            Digit { pos, value }
        })
        .chain(
            [
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]
            .into_iter()
            .flat_map(|(name, value)| {
                s.match_indices(name)
                    .map(move |(pos, _)| Digit { pos, value })
            }),
        )
        .minmax();
    match r {
        MinMaxResult::NoElements => panic!("No digits in line"),
        MinMaxResult::OneElement(Digit { value, .. }) => value * 10 + value,
        MinMaxResult::MinMax(Digit { value: first, .. }, Digit { value: last, .. }) => {
            first * 10 + last
        }
    }
}

fn solve(input: Input) -> u32 {
    input.lines().map(|s| line2number(&s)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen\n",
        ));
        assert_eq!(solve(input), 281);
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn line2number(#[case] line: &str, #[case] number: u32) {
        assert_eq!(super::line2number(line), number);
    }
}
