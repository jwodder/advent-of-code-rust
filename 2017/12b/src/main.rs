use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{components, Input};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProgramSpec {
    id: usize,
    pipes_to: Vec<usize>,
}

impl std::str::FromStr for ProgramSpec {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<ProgramSpec, ParseError> {
        let mut parser = PullParser::new(s);
        let id = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("<-> ")?;
        let pipes_to = parser.delimited(", ", |t| t.parse::<usize>().map_err(ParseError::from))?;
        Ok(ProgramSpec { id, pipes_to })
    }
}

fn solve(input: Input) -> usize {
    let mut ids = Vec::new();
    let mut ids2pipes = std::collections::HashMap::new();
    for spec in input.parse_lines::<ProgramSpec>() {
        ids.push(spec.id);
        ids2pipes.insert(spec.id, spec.pipes_to);
    }
    components(ids, move |a| ids2pipes.remove(&a).unwrap()).len()
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
            "0 <-> 2\n",
            "1 <-> 1\n",
            "2 <-> 0, 3, 4\n",
            "3 <-> 2, 4\n",
            "4 <-> 2, 3, 6\n",
            "5 <-> 6\n",
            "6 <-> 4, 5\n",
        ));
        assert_eq!(solve(input), 2);
    }
}
