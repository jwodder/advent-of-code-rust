use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Step = char;

struct Precondition {
    before: Step,
    after: Step,
}

impl FromStr for Precondition {
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

fn toposort<I: IntoIterator<Item = Precondition>>(iter: I) -> String {
    let mut preconds: HashMap<Step, HashSet<Step>> = HashMap::new();
    for p in iter {
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
    println!(
        "{}",
        toposort(Input::from_env().parse_lines::<Precondition>())
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let preconds = [
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
        .into_iter()
        .map(|s| s.parse::<Precondition>().unwrap());
        assert_eq!(toposort(preconds), "CABDFE");
    }
}
