use adventutil::Input;
use adventutil::counter::Counter;
use adventutil::maxn::maxn;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::str::FromStr;

struct Monkey {
    index: usize,
    items: Vec<u64>,
    op: Operation,
    test_divisor: u64,
    test_true: usize,
    test_false: usize,
}

impl FromStr for Monkey {
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
            .map(|t| t.trim().parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        parser.skip(Token::Whitespace)?;
        parser.skip("Operation: new = old ")?;
        let op = parser.parse_to::<Operation, _>("\n")?;
        parser.skip(Token::Whitespace)?;
        parser.skip("Test: divisible by ")?;
        let test_divisor = parser.parse_to::<u64, _>(Token::Whitespace)?;
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

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Operation::Add(arg) => value + *arg,
            Operation::Mul(arg) => value * *arg,
            Operation::Square => value * value,
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Operation, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("+ ").is_ok() {
            let arg = parser.parse_to::<u64, _>(Token::Eof)?;
            Ok(Operation::Add(arg))
        } else {
            parser.skip("* ")?;
            if parser.skip("old").is_ok() {
                parser.eof()?;
                Ok(Operation::Square)
            } else {
                let arg = parser.parse_to::<u64, _>(Token::Eof)?;
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
    let modulus = monkeys
        .iter()
        .fold(1, |accum, m| lcm(accum, m.test_divisor));
    let mut inspected = Counter::new();
    for _ in 0..10000 {
        for i in 0..(monkeys.len()) {
            let items = monkeys[i].items.drain(..).collect::<Vec<_>>();
            for mut worry in items {
                inspected.add(i);
                worry = monkeys[i].op.apply(worry) % modulus;
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

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(x: u64, y: u64) -> u64 {
    let d = gcd(x, y);
    if d == 0 { 0 } else { x * y / d }
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
        assert_eq!(solve(input), 2713310158);
    }
}
