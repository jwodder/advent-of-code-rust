use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{FromBits, Input};
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Gate {
    op: Op,
    input1: String,
    input2: String,
    output: String,
}

impl Gate {
    // Returns `true` if an evaluation took place
    fn evaluate(&self, wires: &mut BTreeMap<String, bool>) -> bool {
        let Some(&a) = wires.get(&self.input1) else {
            return false;
        };
        let Some(&b) = wires.get(&self.input2) else {
            return false;
        };
        let c = self.op.evaluate(a, b);
        wires.insert(self.output.clone(), c);
        true
    }
}

impl std::str::FromStr for Gate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Gate, ParseError> {
        let mut parser = PullParser::new(s);
        let input1 = parser.parse_to::<String, _>(Token::Whitespace)?;
        let op = match parser.scan_to(Token::Whitespace)? {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            s => return Err(ParseError::InvalidToken(s.to_owned())),
        };
        let input2 = parser.parse_to::<String, _>(Token::Whitespace)?;
        parser.skip("->")?;
        parser.skip(Token::Whitespace)?;
        let output = parser.parse_to::<String, _>(Token::Eof)?;
        Ok(Gate {
            op,
            input1,
            input2,
            output,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn evaluate(self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a & b,
            Op::Or => a | b,
            Op::Xor => a ^ b,
        }
    }
}

fn solve(input: Input) -> u64 {
    let (initial_wires, gates) = input
        .paragraphs()
        .collect_tuple()
        .expect("Input is not exactly two paragraphs");
    let mut wires = BTreeMap::new();
    for ln in initial_wires.lines() {
        let (name, value) = ln.split_once(": ").unwrap();
        let value = match value {
            "0" => false,
            "1" => true,
            s => panic!("Unexpected wire value: {s:?}"),
        };
        wires.insert(name.to_owned(), value);
    }
    let mut gates = gates
        .lines()
        .map(|s| s.parse::<Gate>().unwrap())
        .collect::<Vec<_>>();
    while !gates.is_empty() {
        gates.retain(|g| !g.evaluate(&mut wires));
    }
    let zeds = wires
        .into_iter()
        .filter(|(name, _)| name.starts_with('z'))
        .map(|(name, value)| (std::cmp::Reverse(name), value))
        .collect::<BTreeMap<_, _>>();
    u64::from_bits(zeds.into_values())
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
            "x00: 1\n",
            "x01: 1\n",
            "x02: 1\n",
            "y00: 0\n",
            "y01: 1\n",
            "y02: 0\n",
            "\n",
            "x00 AND y00 -> z00\n",
            "x01 XOR y01 -> z01\n",
            "x02 OR y02 -> z02\n",
        ));
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "x00: 1\n",
            "x01: 0\n",
            "x02: 1\n",
            "x03: 1\n",
            "x04: 0\n",
            "y00: 1\n",
            "y01: 1\n",
            "y02: 1\n",
            "y03: 1\n",
            "y04: 1\n",
            "\n",
            "ntg XOR fgs -> mjb\n",
            "y02 OR x01 -> tnw\n",
            "kwq OR kpj -> z05\n",
            "x00 OR x03 -> fst\n",
            "tgd XOR rvg -> z01\n",
            "vdt OR tnw -> bfw\n",
            "bfw AND frj -> z10\n",
            "ffh OR nrd -> bqk\n",
            "y00 AND y03 -> djm\n",
            "y03 OR y00 -> psh\n",
            "bqk OR frj -> z08\n",
            "tnw OR fst -> frj\n",
            "gnj AND tgd -> z11\n",
            "bfw XOR mjb -> z00\n",
            "x03 OR x00 -> vdt\n",
            "gnj AND wpb -> z02\n",
            "x04 AND y00 -> kjc\n",
            "djm OR pbm -> qhw\n",
            "nrd AND vdt -> hwm\n",
            "kjc AND fst -> rvg\n",
            "y04 OR y02 -> fgs\n",
            "y01 AND x02 -> pbm\n",
            "ntg OR kjc -> kwq\n",
            "psh XOR fgs -> tgd\n",
            "qhw XOR tgd -> z09\n",
            "pbm OR djm -> kpj\n",
            "x03 XOR y03 -> ffh\n",
            "x00 XOR y04 -> ntg\n",
            "bfw OR bqk -> z06\n",
            "nrd XOR fgs -> wpb\n",
            "frj XOR qhw -> z04\n",
            "bqk OR frj -> z07\n",
            "y03 OR x01 -> nrd\n",
            "hwm AND bqk -> z03\n",
            "tgd XOR rvg -> z12\n",
            "tnw OR pbm -> gnj\n",
        ));
        assert_eq!(solve(input), 2024);
    }
}
