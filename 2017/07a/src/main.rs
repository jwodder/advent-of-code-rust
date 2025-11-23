use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl std::str::FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Program, ParseError> {
        let mut parser = PullParser::new(s);
        let name = parser.scan_to(Token::Whitespace)?.to_owned();
        parser.skip('(')?;
        let weight = parser.parse_to::<u32, _>(')')?;
        let children = if parser.eof().is_err() {
            parser.skip(" -> ")?;
            parser.delimited(", ", |p| Ok(p.to_owned()))?
        } else {
            Vec::new()
        };
        Ok(Program {
            name,
            weight,
            children,
        })
    }
}

fn solve(input: Input) -> String {
    let mut names = HashSet::new();
    let mut parented = HashSet::new();
    for program in input.parse_lines::<Program>() {
        names.insert(program.name);
        parented.extend(program.children);
    }
    let Some((root,)) = names.difference(&parented).collect_tuple() else {
        panic!("len(names - parented) != 1");
    };
    root.clone()
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
            "pbga (66)\n",
            "xhth (57)\n",
            "ebii (61)\n",
            "havc (66)\n",
            "ktlj (57)\n",
            "fwft (72) -> ktlj, cntj, xhth\n",
            "qoyq (66)\n",
            "padx (45) -> pbga, havc, qoyq\n",
            "tknk (41) -> ugml, padx, fwft\n",
            "jptl (61)\n",
            "ugml (68) -> gyxo, ebii, jptl\n",
            "gyxo (61)\n",
            "cntj (57)\n",
        ));
        assert_eq!(solve(input), "tknk");
    }
}
