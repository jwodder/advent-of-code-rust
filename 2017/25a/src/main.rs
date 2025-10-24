use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Header {
    initial_state: char,
    steps: usize,
}

impl std::str::FromStr for Header {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Header, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Begin in state ")?;
        let initial_state = parser.parse_to::<char, _>('.')?;
        parser.skip(Token::Newline)?;
        parser.skip("Perform a diagnostic checksum after ")?;
        let steps = parser.parse_to::<usize, _>(" steps.")?;
        parser.eof()?;
        Ok(Header {
            initial_state,
            steps,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StateSpec {
    name: char,
    on_zero: Transition,
    on_one: Transition,
}

impl std::str::FromStr for StateSpec {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<StateSpec, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("In state ")?;
        let name = parser.parse_to::<char, _>(':')?;
        parser.skip(Token::Newline)?;

        parser.skip("  If the current value is 0:")?;
        parser.skip(Token::Newline)?;
        parser.skip("    - Write the value ")?;
        let zero_write = match parser.scan_to('.')? {
            "0" => false,
            "1" => true,
            t => return Err(ParseError::InvalidToken(t.to_owned())),
        };
        parser.skip(Token::Newline)?;
        parser.skip("    - Move one slot to the ")?;
        let zero_motion = parser.parse_to::<Motion, _>('.')?;
        parser.skip(Token::Newline)?;
        parser.skip("    - Continue with state ")?;
        let zero_state = parser.parse_to::<char, _>('.')?;
        parser.skip(Token::Newline)?;

        parser.skip("  If the current value is 1:")?;
        parser.skip(Token::Newline)?;
        parser.skip("    - Write the value ")?;
        let one_write = match parser.scan_to('.')? {
            "0" => false,
            "1" => true,
            t => return Err(ParseError::InvalidToken(t.to_owned())),
        };
        parser.skip(Token::Newline)?;
        parser.skip("    - Move one slot to the ")?;
        let one_motion = parser.parse_to::<Motion, _>('.')?;
        parser.skip(Token::Newline)?;
        parser.skip("    - Continue with state ")?;
        let one_state = parser.parse_to::<char, _>('.')?;
        parser.eof()?;

        Ok(StateSpec {
            name,
            on_zero: Transition {
                write: zero_write,
                motion: zero_motion,
                new_state: zero_state,
            },
            on_one: Transition {
                write: one_write,
                motion: one_motion,
                new_state: one_state,
            },
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Transition {
    write: bool,
    motion: Motion,
    new_state: char,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Motion {
    Left,
    Right,
}

impl std::str::FromStr for Motion {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Motion, ParseError> {
        match s {
            "left" => Ok(Motion::Left),
            "right" => Ok(Motion::Right),
            _ => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

fn solve(input: Input) -> usize {
    let mut paras = input.paragraphs();
    let header = paras.next().unwrap().parse::<Header>().unwrap();
    let program = paras
        .map(|p| {
            let spec = p.parse::<StateSpec>().unwrap();
            (spec.name, (spec.on_zero, spec.on_one))
        })
        .collect::<HashMap<_, _>>();
    // Indices of 1-valued cells on tape:
    let mut ones: HashSet<i32> = HashSet::new();
    let mut state = header.initial_state;
    let mut cursor = 0i32;
    for _ in 0..header.steps {
        let (on_zero, on_one) = program.get(&state).copied().unwrap();
        let transition = if ones.contains(&cursor) {
            on_one
        } else {
            on_zero
        };
        if transition.write {
            ones.insert(cursor);
        } else {
            ones.remove(&cursor);
        }
        match transition.motion {
            Motion::Left => cursor -= 1,
            Motion::Right => cursor += 1,
        }
        state = transition.new_state;
    }
    ones.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
