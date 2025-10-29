// For simplicity, we assume the following:
// - When matching a rule with alternation, there will never be more than one
//   matching branch.
use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type RuleMap = HashMap<usize, Rule>;

enum Rule {
    Terminal(char),
    Reference(usize),
    Sequence(Vec<Rule>),
    Alternation(Vec<Rule>),
}

impl Rule {
    fn matches(&self, s: &str, rules: &RuleMap) -> bool {
        let mut parser = PullParser::new(s);
        self.parses(&mut parser, rules) && parser.eof().is_ok()
    }

    fn parses(&self, parser: &mut PullParser<'_>, rules: &RuleMap) -> bool {
        match self {
            Rule::Terminal(c) => parser.skip(*c).is_ok(),
            Rule::Reference(idx) => rules[idx].parses(parser, rules),
            Rule::Sequence(seq) => {
                for r in seq {
                    if !r.parses(parser, rules) {
                        return false;
                    }
                }
                true
            }
            Rule::Alternation(alts) => {
                for r in alts {
                    let backup = *parser;
                    if r.parses(parser, rules) {
                        return true;
                    } else {
                        *parser = backup;
                    }
                }
                false
            }
        }
    }
}

struct IndexedRule {
    index: usize,
    rule: Rule,
}

impl IndexedRule {
    fn into_pair(self) -> (usize, Rule) {
        (self.index, self.rule)
    }
}

impl FromStr for IndexedRule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<IndexedRule, ParseError> {
        fn parse_sequence(s: &str) -> Result<Rule, ParseError> {
            let refs = s
                .split_ascii_whitespace()
                .map(|word| word.parse::<usize>().map(Rule::Reference))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Rule::Sequence(refs))
        }

        let mut parser = PullParser::new(s);
        let index = parser.parse_to::<usize, _>(':')?;
        parser.skip(Token::Whitespace)?;
        let rule = if parser.skip('"').is_ok() {
            let c = parser.parse_to::<char, _>('"')?;
            parser.eof()?;
            Rule::Terminal(c)
        } else {
            let alts = parser.delimited(" | ", parse_sequence)?;
            Rule::Alternation(alts)
        };
        Ok(IndexedRule { index, rule })
    }
}

fn solve(input: Input) -> usize {
    let (rules, messages) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let rules = rules
        .lines()
        .map(|s| s.parse::<IndexedRule>().expect("Parse error").into_pair())
        .collect::<RuleMap>();
    let top = rules.get(&0).expect("No rule 0");
    messages.lines().filter(|s| top.matches(s, &rules)).count()
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
            "0: 4 1 5\n",
            "1: 2 3 | 3 2\n",
            "2: 4 4 | 5 5\n",
            "3: 4 5 | 5 4\n",
            "4: \"a\"\n",
            "5: \"b\"\n",
            "\n",
            "ababbb\n",
            "bababa\n",
            "abbbab\n",
            "aaabbb\n",
            "aaaabbb\n",
        ));
        assert_eq!(solve(input), 2);
    }
}
