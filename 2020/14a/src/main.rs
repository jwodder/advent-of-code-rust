use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    SetMask(Vec<Option<bool>>),
    SetMem(u64, u64),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("mask = ").is_ok() {
            let mut mask = Vec::new();
            for c in parser.into_str().chars() {
                match c {
                    'X' => mask.push(None),
                    '1' => mask.push(Some(true)),
                    '0' => mask.push(Some(false)),
                    c => return Err(ParseError::InvalidToken(c.to_string())),
                }
            }
            if mask.len() != 36 {
                return Err(ParseError::InvalidToken("mask not 36 bits".into()));
            }
            Ok(Instruction::SetMask(mask))
        } else {
            parser.skip("mem[")?;
            let address = parser.parse_to::<u64, _>(']')?;
            parser.skip(" = ")?;
            let value = parser.parse_to::<u64, _>(Token::Eof)?;
            Ok(Instruction::SetMem(address, value))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    mask: Vec<Option<bool>>,
    memory: HashMap<u64, u64>,
}

impl State {
    fn new() -> State {
        State {
            mask: vec![None; 36],
            memory: HashMap::new(),
        }
    }

    fn set_mask(&mut self, mask: Vec<Option<bool>>) {
        self.mask = mask;
    }

    fn set_memory(&mut self, address: u64, value: u64) {
        self.memory.insert(address, apply_mask(&self.mask, value));
    }

    fn sum(self) -> u64 {
        self.memory.into_values().sum()
    }
}

fn apply_mask(mask: &[Option<bool>], mut value: u64) -> u64 {
    for (i, &b) in mask.iter().rev().enumerate() {
        match b {
            Some(true) => value |= 1 << i,
            Some(false) => value &= !(1 << i),
            None => (),
        }
    }
    value
}

fn solve(input: Input) -> u64 {
    let mut state = State::new();
    for instruction in input.parse_lines::<Instruction>() {
        match instruction {
            Instruction::SetMask(mask) => state.set_mask(mask),
            Instruction::SetMem(address, value) => state.set_memory(address, value),
        }
    }
    state.sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(11, 73)]
    #[case(101, 101)]
    #[case(0, 64)]
    fn apply_mask(#[case] value: u64, #[case] masked: u64) {
        let mask = [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(true),
            None,
            None,
            None,
            None,
            Some(false),
            None,
        ];
        assert_eq!(super::apply_mask(&mask[..], value), masked);
    }

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n",
            "mem[8] = 11\n",
            "mem[7] = 101\n",
            "mem[8] = 0\n",
        ));
        assert_eq!(solve(input), 165);
    }
}
