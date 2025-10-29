use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashMap;
use std::str::FromStr;

struct State {
    // Mapping of connected wires to their assigned values
    assigned: HashMap<Wire, u16>,
}

impl State {
    fn new() -> State {
        State {
            assigned: HashMap::new(),
        }
    }

    fn get(&self, wire: &Wire) -> Option<u16> {
        self.assigned.get(wire).copied()
    }

    fn has(&self, wire: &Wire) -> bool {
        self.assigned.contains_key(wire)
    }

    fn assign(&mut self, wire: &Wire, value: u16) {
        self.assigned.insert(wire.clone(), value);
    }

    fn evaluate(&self, won: &WireOrNum) -> Option<u16> {
        match won {
            WireOrNum::Wire(w) => self.get(w),
            WireOrNum::Num(n) => Some(*n),
        }
    }
}

fn solve(input: Input) -> u16 {
    let mut state = State::new();
    let mut unfollowed = input
        .parse_lines::<Instruction>()
        .map(|i| (i.output().clone(), i))
        .collect::<HashMap<Wire, Instruction>>();
    let target = Wire("a".into());
    while !state.has(&target) {
        let ready = unfollowed
            .iter()
            .filter_map(|(w, i)| i.operate(&state).map(|v| (w.clone(), v)))
            .collect::<Vec<_>>();
        assert!(!ready.is_empty(), "Nothing to connect");
        for (w, value) in ready {
            state.assign(&w, value);
            unfollowed.remove(&w);
        }
    }
    state.get(&target).unwrap()
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Copy {
        input: WireOrNum,
        output: Wire,
    },
    Not {
        input: WireOrNum,
        output: Wire,
    },
    And {
        left: WireOrNum,
        right: WireOrNum,
        output: Wire,
    },
    Or {
        left: WireOrNum,
        right: WireOrNum,
        output: Wire,
    },
    LShift {
        left: WireOrNum,
        right: WireOrNum,
        output: Wire,
    },
    RShift {
        left: WireOrNum,
        right: WireOrNum,
        output: Wire,
    },
}

impl Instruction {
    fn output(&self) -> &Wire {
        use Instruction::*;
        match self {
            Copy { output, .. } => output,
            Not { output, .. } => output,
            And { output, .. } => output,
            Or { output, .. } => output,
            LShift { output, .. } => output,
            RShift { output, .. } => output,
        }
    }

    fn operate(&self, state: &State) -> Option<u16> {
        use Instruction::*;
        match self {
            Copy { input, .. } => state.evaluate(input),
            Not { input, .. } => state.evaluate(input).map(|v| !v),
            And { left, right, .. } => {
                let left = state.evaluate(left)?;
                let right = state.evaluate(right)?;
                Some(left & right)
            }
            Or { left, right, .. } => {
                let left = state.evaluate(left)?;
                let right = state.evaluate(right)?;
                Some(left | right)
            }
            LShift { left, right, .. } => {
                let left = state.evaluate(left)?;
                let right = state.evaluate(right)?;
                Some(left << right)
            }
            RShift { left, right, .. } => {
                let left = state.evaluate(left)?;
                let right = state.evaluate(right)?;
                Some(left >> right)
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("NOT ").is_ok() {
            let input = parser.parse_to::<WireOrNum, _>(Token::Whitespace)?;
            parser.skip("-> ")?;
            let output = parser.parse_to::<Wire, _>(Token::Eof)?;
            return Ok(Instruction::Not { input, output });
        }
        let left = parser.parse_to::<WireOrNum, _>(Token::Whitespace)?;
        if parser.skip("-> ").is_ok() {
            let output = parser.parse_to::<Wire, _>(Token::Eof)?;
            return Ok(Instruction::Copy {
                input: left,
                output,
            });
        }
        let opname = parser.scan_to(Token::Whitespace)?;
        let right = parser.parse_to::<WireOrNum, _>(Token::Whitespace)?;
        parser.skip("-> ")?;
        let output = parser.parse_to::<Wire, _>(Token::Eof)?;
        match opname {
            "AND" => Ok(Instruction::And {
                left,
                right,
                output,
            }),
            "OR" => Ok(Instruction::Or {
                left,
                right,
                output,
            }),
            "LSHIFT" => Ok(Instruction::LShift {
                left,
                right,
                output,
            }),
            "RSHIFT" => Ok(Instruction::RShift {
                left,
                right,
                output,
            }),
            op => Err(ParseError::InvalidToken(op.into())),
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Wire(String);

impl FromStr for Wire {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Wire, ParseError> {
        if s.chars().all(|c| c.is_ascii_lowercase()) {
            Ok(Wire(s.into()))
        } else {
            Err(ParseError::InvalidToken(s.into()))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WireOrNum {
    Wire(Wire),
    Num(u16),
}

impl FromStr for WireOrNum {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<WireOrNum, ParseError> {
        match s.parse::<Wire>() {
            Ok(w) => Ok(WireOrNum::Wire(w)),
            Err(_) => Ok(WireOrNum::Num(s.parse::<u16>()?)),
        }
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    use rstest::rstest;

    #[rstest]
    #[case("x AND y -> z", And {left: WireOrNum::Wire(Wire("x".into())), right: WireOrNum::Wire(Wire("y".into())), output: Wire("z".into())})]
    #[case("123 -> x", Copy {input: WireOrNum::Num(123), output: Wire("x".into())})]
    #[case("p LSHIFT 2 -> q", LShift {left: WireOrNum::Wire(Wire("p".into())), right: WireOrNum::Num(2), output: Wire("q".into())})]
    #[case("NOT e -> f", Not {input: WireOrNum::Wire(Wire("e".into())), output: Wire("f".into())})]
    #[case("x OR y -> e", Or {left: WireOrNum::Wire(Wire("x".into())), right: WireOrNum::Wire(Wire("y".into())), output: Wire("e".into())})]
    #[case("y RSHIFT 2 -> g", RShift {left: WireOrNum::Wire(Wire("y".into())), right: WireOrNum::Num(2), output: Wire("g".into())})]
    fn test_parse_instruction(#[case] s: &str, #[case] inst: Instruction) {
        assert_eq!(s.parse::<Instruction>().unwrap(), inst);
    }
}
