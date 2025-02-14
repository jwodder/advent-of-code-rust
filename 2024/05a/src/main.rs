use adventutil::{parse_csv, Input};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rule {
    before: u32,
    after: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Update {
    page_positions: HashMap<u32, usize>,
    middle: u32,
}

impl Update {
    fn satisfies(&self, rule: &Rule) -> bool {
        self.page_positions
            .get(&rule.before)
            .zip(self.page_positions.get(&rule.after))
            .is_none_or(|(i, j)| i < j)
    }
}

impl From<Vec<u32>> for Update {
    fn from(values: Vec<u32>) -> Update {
        let middle = values[values.len() / 2];
        let page_positions = std::iter::zip(values, 0usize..).collect();
        Update {
            page_positions,
            middle,
        }
    }
}

fn solve(input: Input) -> u32 {
    let mut paraiter = input.paragraphs();
    let rules = paraiter
        .next()
        .unwrap()
        .lines()
        .map(|ln| {
            let (before, after) = ln.split_once('|').unwrap();
            let before = before.parse::<u32>().unwrap();
            let after = after.parse::<u32>().unwrap();
            Rule { before, after }
        })
        .collect::<Vec<_>>();
    paraiter
        .next()
        .unwrap()
        .lines()
        .map(|ln| Update::from(parse_csv::<u32>(ln)))
        .filter(|update| rules.iter().all(|r| update.satisfies(r)))
        .map(|update| update.middle)
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "47|53\n",
            "97|13\n",
            "97|61\n",
            "97|47\n",
            "75|29\n",
            "61|13\n",
            "75|53\n",
            "29|13\n",
            "97|29\n",
            "53|29\n",
            "61|53\n",
            "97|53\n",
            "61|29\n",
            "47|13\n",
            "75|47\n",
            "97|75\n",
            "47|61\n",
            "75|61\n",
            "47|29\n",
            "75|13\n",
            "53|13\n",
            "\n",
            "75,47,61,53,29\n",
            "97,61,53,29,13\n",
            "75,29,13\n",
            "75,97,47,61,53\n",
            "61,13,29\n",
            "97,13,75,29,47\n",
        ));
        assert_eq!(solve(input), 143);
    }
}
