use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Give {
    chip: u32,
    bot: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Split {
    bot: u32,
    low_target: Destination,
    high_target: Destination,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Destination {
    Output(u32),
    Bot(u32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Give(Give),
    Split(Split),
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        if parser.skip("value ").is_ok() {
            let chip = parser.parse_to::<u32, _>(Token::Whitespace)?;
            parser.skip("goes to bot ")?;
            let bot = parser.parse_to::<u32, _>(Token::Eof)?;
            Ok(Instruction::Give(Give { chip, bot }))
        } else {
            parser.skip("bot ")?;
            let bot = parser.parse_to::<u32, _>(Token::Whitespace)?;
            parser.skip("gives low to ")?;
            let low_target = if parser.skip("bot ").is_ok() {
                Destination::Bot(parser.parse_to::<u32, _>(Token::Whitespace)?)
            } else {
                parser.skip("output ")?;
                Destination::Output(parser.parse_to::<u32, _>(Token::Whitespace)?)
            };
            parser.skip("and high to ")?;
            let high_target = if parser.skip("bot ").is_ok() {
                Destination::Bot(parser.parse_to::<u32, _>(Token::Eof)?)
            } else {
                parser.skip("output ")?;
                Destination::Output(parser.parse_to::<u32, _>(Token::Eof)?)
            };
            Ok(Instruction::Split(Split {
                bot,
                low_target,
                high_target,
            }))
        }
    }
}

fn solve(input: Input) -> u32 {
    let mut holding = HashMap::<u32, Vec<u32>>::new();
    let mut splits = HashMap::new();
    let mut givings = VecDeque::new();
    let mut output = HashMap::new();
    for instr in input.parse_lines::<Instruction>() {
        match instr {
            Instruction::Give(g) => givings.push_back(g),
            Instruction::Split(sp) => {
                splits.insert(sp.bot, sp);
            }
        }
    }
    while let Some(g) = givings.pop_front() {
        let held = holding.entry(g.bot).or_default();
        held.push(g.chip);
        if held.len() == 2 {
            let x = held.pop().unwrap();
            let y = held.pop().unwrap();
            let high_chip = x.max(y);
            let low_chip = x.min(y);
            let sp = splits[&g.bot];
            match sp.low_target {
                Destination::Bot(b) => givings.push_back(Give {
                    chip: low_chip,
                    bot: b,
                }),
                Destination::Output(o) => {
                    output.insert(o, low_chip);
                }
            }
            match sp.high_target {
                Destination::Bot(b) => givings.push_back(Give {
                    chip: high_chip,
                    bot: b,
                }),
                Destination::Output(o) => {
                    output.insert(o, high_chip);
                }
            }
        }
    }
    output[&0] * output[&1] * output[&2]
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
