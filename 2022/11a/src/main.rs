use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::maxn::maxn;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Monkey {
    index: usize,
    items: Vec<u32>,
    op: Operation,
    test_divisor: u32,
    test_true: usize,
    test_false: usize,
}

impl std::str::FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Monkey, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Monkey ")?;
        let index = parser.parse_to::<usize, _>(':')?;
        parser.skip(Token::Whitespace)?;
        parser.skip("Starting items: ")?;
        let items = parser
            .scan_to(Token::Newline)?
            .split(',')
            .map(|t| t.trim().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        parser.skip(Token::Whitespace)?;
        parser.skip("Operation: new = old ")?;
        let op = parser.parse_to::<Operation, _>("\n")?;
        parser.skip(Token::Whitespace)?;
        parser.skip("Test: divisible by ")?;
        let test_divisor = parser.parse_to::<u32, _>(Token::Whitespace)?;
        parser.skip("If true: throw to monkey ")?;
        let test_true = parser.parse_to::<usize, _>(Token::Whitespace)?;
        parser.skip("If false: throw to monkey ")?;
        let test_false = parser.parse_to::<usize, _>(Token::Eof)?;
        Ok(Monkey {
            index,
            items,
            op,
            test_divisor,
            test_true,
            test_false,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

impl Operation {
    fn apply(&self, value: u32) -> u32 {
        match self {
            Operation::Add(arg) => value + *arg,
            Operation::Mul(arg) => value * *arg,
            Operation::Square => value * value,
        }
    }
}

impl std::str::FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operation, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("+ ").is_ok() {
            let arg = parser.parse_to::<u32, _>(Token::Eof)?;
            Ok(Operation::Add(arg))
        } else {
            parser.skip("* ")?;
            if parser.skip("old").is_ok() {
                parser.eof()?;
                Ok(Operation::Square)
            } else {
                let arg = parser.parse_to::<u32, _>(Token::Eof)?;
                Ok(Operation::Mul(arg))
            }
        }
    }
}

fn solve(input: Input) -> u64 {
    let mut monkeys = input
        .paragraphs()
        .map(|p| p.parse::<Monkey>().expect("Parse error"))
        .collect::<Vec<_>>();
    for (i, m) in monkeys.iter().enumerate() {
        assert_eq!(m.index, i);
    }
    let mut inspected = Counter::new();
    for _ in 0..20 {
        for i in 0..(monkeys.len()) {
            let items = monkeys[i].items.drain(..).collect::<Vec<_>>();
            for mut worry in items {
                inspected.add(i);
                worry = monkeys[i].op.apply(worry) / 3;
                let target = if worry % monkeys[i].test_divisor == 0 {
                    monkeys[i].test_true
                } else {
                    monkeys[i].test_false
                };
                monkeys[target].items.push(worry);
            }
        }
    }
    maxn(2, inspected.into_values()).into_iter().product()
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
            "Monkey 0:\n",
            "  Starting items: 79, 98\n",
            "  Operation: new = old * 19\n",
            "  Test: divisible by 23\n",
            "    If true: throw to monkey 2\n",
            "    If false: throw to monkey 3\n",
            "\n",
            "Monkey 1:\n",
            "  Starting items: 54, 65, 75, 74\n",
            "  Operation: new = old + 6\n",
            "  Test: divisible by 19\n",
            "    If true: throw to monkey 2\n",
            "    If false: throw to monkey 0\n",
            "\n",
            "Monkey 2:\n",
            "  Starting items: 79, 60, 97\n",
            "  Operation: new = old * old\n",
            "  Test: divisible by 13\n",
            "    If true: throw to monkey 1\n",
            "    If false: throw to monkey 3\n",
            "\n",
            "Monkey 3:\n",
            "  Starting items: 74\n",
            "  Operation: new = old + 3\n",
            "  Test: divisible by 17\n",
            "    If true: throw to monkey 0\n",
            "    If false: throw to monkey 1\n",
        ));
        assert_eq!(solve(input), 10605);
    }
}
