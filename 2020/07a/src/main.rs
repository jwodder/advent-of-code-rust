use adventutil::closure::one2many_closure;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Color(String);

struct Relation {
    container: Color,
    contents: Vec<Color>,
}

impl FromStr for Relation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Relation, ParseError> {
        let mut parser = PullParser::new(s);
        let container = Color(parser.scan_to(" bags contain ")?.into());
        let mut contents = Vec::new();
        let s = parser.into_str();
        if s != "no other bags." {
            for s in s.split(", ") {
                let mut parser = PullParser::new(s);
                let _qty = parser.parse_to::<usize, _>(Token::Whitespace)?;
                let color = Color(parser.scan_to(" bag")?.into());
                let _ = parser.skip("s");
                let _ = parser.skip(".");
                parser.eof()?;
                contents.push(color);
            }
        }
        Ok(Relation {
            container,
            contents,
        })
    }
}

fn solve(input: Input) -> usize {
    // Mapping from Colors to the Colors that can contain them:
    let relationship = input
        .parse_lines::<Relation>()
        .flat_map(|rel| {
            rel.contents
                .into_iter()
                .map(move |c| (c, rel.container.clone()))
        })
        .into_group_map();
    one2many_closure(Color("shiny gold".into()), |color| {
        relationship.get(&color).cloned().unwrap_or_default()
    })
    .len()
        - 1
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
            "light red bags contain 1 bright white bag, 2 muted yellow bags.\n",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n",
            "bright white bags contain 1 shiny gold bag.\n",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n",
            "faded blue bags contain no other bags.\n",
            "dotted black bags contain no other bags.\n",
        ));
        assert_eq!(solve(input), 4);
    }
}
