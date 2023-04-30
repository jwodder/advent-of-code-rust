use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Color(String);

struct Relation {
    container: Color,
    contents: Vec<(usize, Color)>,
}

impl Relation {
    fn into_pair(self) -> (Color, Vec<(usize, Color)>) {
        (self.container, self.contents)
    }
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
                let qty = parser.parse_to::<usize, _>(Token::Whitespace)?;
                let color = Color(parser.scan_to(" bag")?.into());
                let _ = parser.skip("s");
                let _ = parser.skip(".");
                parser.eof()?;
                contents.push((qty, color));
            }
        }
        Ok(Relation {
            container,
            contents,
        })
    }
}

fn solve(input: Input) -> usize {
    let relationship: HashMap<Color, Vec<(usize, Color)>> = input
        .parse_lines::<Relation>()
        .map(Relation::into_pair)
        .collect();
    contents(&relationship, &Color("shiny gold".into()))
}

fn contents(relationship: &HashMap<Color, Vec<(usize, Color)>>, color: &Color) -> usize {
    match relationship.get(color) {
        Some(in_color) => in_color
            .iter()
            .map(|(qty, subcolor)| qty + qty * contents(relationship, subcolor))
            .sum(),
        None => 0,
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
        assert_eq!(solve(input), 32);
    }

    #[test]
    fn test_example2() {
        let input = Input::from(concat!(
            "shiny gold bags contain 2 dark red bags.\n",
            "dark red bags contain 2 dark orange bags.\n",
            "dark orange bags contain 2 dark yellow bags.\n",
            "dark yellow bags contain 2 dark green bags.\n",
            "dark green bags contain 2 dark blue bags.\n",
            "dark blue bags contain 2 dark violet bags.\n",
            "dark violet bags contain no other bags.\n",
        ));
        assert_eq!(solve(input), 126);
    }
}
