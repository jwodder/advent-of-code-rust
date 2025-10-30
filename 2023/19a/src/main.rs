use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: Part) -> Target {
        for rule in &self.rules {
            if let Some(t) = rule.apply(part) {
                return t;
            }
        }
        panic!("No rule in workflow {} matched {part:?}", self.name);
    }
}

impl std::str::FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Workflow, ParseError> {
        let mut parser = PullParser::new(s);
        let name = parser.parse_to::<String, _>('{')?;
        let inner = parser.scan_to('}')?;
        parser.eof()?;
        let rules = PullParser::new(inner).delimited(",", Rule::from_str)?;
        Ok(Workflow { name, rules })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Rule {
    Compare {
        category: Category,
        op: Op,
        bound: u32,
        target: Target,
    },
    Always(Target),
}

impl Rule {
    fn apply(&self, part: Part) -> Option<Target> {
        match self {
            Rule::Compare {
                category,
                op,
                bound,
                target,
            } => op.eval(part.get(*category), *bound).then(|| target.clone()),
            Rule::Always(target) => Some(target.clone()),
        }
    }
}

impl std::str::FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Rule, ParseError> {
        if s.contains(':') {
            let mut parser = PullParser::new(s);
            let (category, op) = if let Ok(cat) = parser.scan_to('<') {
                (cat.parse::<Category>()?, Op::LessThan)
            } else {
                let cat = parser.scan_to('>')?;
                (cat.parse::<Category>()?, Op::GreaterThan)
            };
            let bound = parser.parse_to::<u32, _>(':')?;
            let target = parser.parse_to::<Target, _>(Token::Eof)?;
            Ok(Rule::Compare {
                category,
                op,
                bound,
                target,
            })
        } else {
            s.parse::<Target>().map(Rule::Always)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get(self, cat: Category) -> u32 {
        match cat {
            Category::Xtreme => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        }
    }

    fn total(self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl std::str::FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Part, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("{x=")?;
        let x = parser.parse_to::<u32, _>(",m=")?;
        let m = parser.parse_to::<u32, _>(",a=")?;
        let a = parser.parse_to::<u32, _>(",s=")?;
        let s = parser.parse_to::<u32, _>("}")?;
        parser.eof()?;
        Ok(Part { x, m, a, s })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Category {
    Xtreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl std::str::FromStr for Category {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Category, ParseError> {
        match s {
            "x" => Ok(Category::Xtreme),
            "m" => Ok(Category::Musical),
            "a" => Ok(Category::Aerodynamic),
            "s" => Ok(Category::Shiny),
            _ => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Target {
    Workflow(String),
    Accept,
    Reject,
}

impl std::str::FromStr for Target {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Target, ParseError> {
        match s {
            "A" => Ok(Target::Accept),
            "R" => Ok(Target::Reject),
            s => Ok(Target::Workflow(s.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    GreaterThan,
    LessThan,
}

impl Op {
    fn eval(self, left: u32, right: u32) -> bool {
        match self {
            Op::GreaterThan => left > right,
            Op::LessThan => left < right,
        }
    }
}

fn solve(input: Input) -> u32 {
    let (workflows, parts) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let workflows = workflows
        .lines()
        .map(|ln| ln.parse::<Workflow>().unwrap())
        .map(|wk| (wk.name.clone(), wk))
        .collect::<HashMap<_, _>>();
    parts
        .lines()
        .map(|ln| ln.parse::<Part>().unwrap())
        .filter(|&part| {
            let mut wkflw = &workflows["in"];
            loop {
                match wkflw.apply(part) {
                    Target::Workflow(name) => wkflw = &workflows[&name],
                    Target::Accept => return true,
                    Target::Reject => return false,
                }
            }
        })
        .map(Part::total)
        .sum()
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
            "px{a<2006:qkq,m>2090:A,rfg}\n",
            "pv{a>1716:R,A}\n",
            "lnx{m>1548:A,A}\n",
            "rfg{s<537:gd,x>2440:R,A}\n",
            "qs{s>3448:A,lnx}\n",
            "qkq{x<1416:A,crn}\n",
            "crn{x>2662:A,R}\n",
            "in{s<1351:px,qqz}\n",
            "qqz{s>2770:qs,m<1801:hdj,R}\n",
            "gd{a>3333:R,R}\n",
            "hdj{m>838:A,pv}\n",
            "\n",
            "{x=787,m=2655,a=1222,s=2876}\n",
            "{x=1679,m=44,a=2067,s=496}\n",
            "{x=2036,m=264,a=79,s=2244}\n",
            "{x=2461,m=1339,a=466,s=291}\n",
            "{x=2127,m=1623,a=2188,s=1013}\n",
        ));
        assert_eq!(solve(input), 19114);
    }
}
