use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::{HashMap, HashSet};

type Step = char;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Precondition {
    before: Step,
    after: Step,
}

impl std::str::FromStr for Precondition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Precondition, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Step ")?;
        let before = parser.parse_to::<Step, _>(Token::Whitespace)?;
        parser.skip("must be finished before step ")?;
        let after = parser.parse_to::<Step, _>(Token::Whitespace)?;
        parser.skip("can begin.")?;
        parser.eof()?;
        Ok(Precondition { before, after })
    }
}

fn solve(input: Input) -> String {
    let mut preconds: HashMap<Step, HashSet<Step>> = HashMap::new();
    for p in input.parse_lines::<Precondition>() {
        preconds.entry(p.after).or_default().insert(p.before);
        preconds.entry(p.before).or_default();
    }
    let mut steps = Vec::new();
    let mut done = HashSet::new();
    while !preconds.is_empty() {
        let &todo = preconds
            .iter()
            .filter_map(|(c, needed)| needed.is_subset(&done).then_some(c))
            .min()
            .expect("No step is ready!");
        steps.push(todo);
        done.insert(todo);
        preconds.remove(&todo);
    }
    steps.into_iter().collect()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "Step C must be finished before step A can begin.\n",
            "Step C must be finished before step F can begin.\n",
            "Step A must be finished before step B can begin.\n",
            "Step A must be finished before step D can begin.\n",
            "Step B must be finished before step E can begin.\n",
            "Step D must be finished before step E can begin.\n",
            "Step F must be finished before step E can begin.\n",
        ));
        assert_eq!(solve(input), "CABDFE");
    }
}
