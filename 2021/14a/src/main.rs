use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::str::FromStr;

struct Rule {
    pair: String,
    insert: char,
}

impl Rule {
    fn into_pair(self) -> (String, char) {
        (self.pair, self.insert)
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Rule, ParseError> {
        let mut parser = PullParser::new(s);
        let pair = parser.parse_to::<String, _>(" -> ")?;
        if pair.chars().count() != 2 {
            return Err(ParseError::InvalidToken(pair));
        }
        let insert = parser.parse_to::<char, _>(Token::Eof)?;
        Ok(Rule { pair, insert })
    }
}

fn solve(input: Input) -> u64 {
    let (mut template, rules) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let rules = rules
        .lines()
        .map(|s| s.parse::<Rule>().expect("Parse error").into_pair())
        .collect::<HashMap<_, _>>();
    for _ in 0..10 {
        let mut i = 0;
        while let Some(s) = template.get(i..(i + 2)) {
            if let Some(c) = rules.get(s) {
                template.insert(i + 1, *c);
                i += 1;
            }
            i += 1;
        }
    }
    match template
        .chars()
        .collect::<Counter<_>>()
        .into_values()
        .minmax()
    {
        MinMaxResult::MinMax(x, y) => y - x,
        _ => 0,
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "NNCB\n",
            "\n",
            "CH -> B\n",
            "HH -> N\n",
            "CB -> H\n",
            "NH -> C\n",
            "HB -> C\n",
            "HC -> B\n",
            "HN -> C\n",
            "NN -> C\n",
            "BH -> H\n",
            "NC -> B\n",
            "NB -> B\n",
            "BN -> B\n",
            "BB -> N\n",
            "BC -> B\n",
            "CC -> N\n",
            "CN -> C\n",
        ));
        assert_eq!(solve(input), 1588);
    }
}
