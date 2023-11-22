use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::HashMap;
use std::str::FromStr;

enum Instruction {
    SetMask(Vec<Option<bool>>),
    SetMem(u64, u64),
}

impl FromStr for Instruction {
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
        for addr in apply_mask(&self.mask, address) {
            self.memory.insert(addr, value);
        }
    }

    fn sum(self) -> u64 {
        self.memory.into_values().sum()
    }
}

fn apply_mask(mask: &[Option<bool>], address: u64) -> Vec<u64> {
    let mut outputs = vec![address];
    for (i, &b) in mask.iter().rev().enumerate() {
        match b {
            Some(true) => {
                for value in &mut outputs {
                    *value |= 1 << i;
                }
            }
            Some(false) => (),
            None => {
                let mut newout = Vec::with_capacity(outputs.len() * 2);
                for value in outputs {
                    newout.push(value & !(1 << i));
                    newout.push(value | (1 << i));
                }
                outputs = newout;
            }
        }
    }
    outputs
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

    #[test]
    fn test_apply_mask_example1() {
        let mask = [
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            None,
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            None,
        ];
        let mut masked = apply_mask(&mask[..], 42);
        masked.sort_unstable();
        assert_eq!(masked, [26, 27, 58, 59]);
    }

    #[test]
    fn test_apply_mask_example2() {
        let mask = [
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            Some(false),
            None,
            Some(false),
            None,
            None,
        ];
        let mut masked = apply_mask(&mask[..], 26);
        masked.sort_unstable();
        assert_eq!(masked, [16, 17, 18, 19, 24, 25, 26, 27]);
    }

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "mask = 000000000000000000000000000000X1001X\n",
            "mem[42] = 100\n",
            "mask = 00000000000000000000000000000000X0XX\n",
            "mem[26] = 1\n",
        ));
        assert_eq!(solve(input), 208);
    }
}
