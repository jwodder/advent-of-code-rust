use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    SwapPos { pos1: usize, pos2: usize },
    SwapChars { ch1: char, ch2: char },
    RotateLeft(usize),
    RotateRight(usize),
    RotatePos(char),
    Reverse { start: usize, end: usize },
    Move { src: usize, dest: usize },
}

impl Instruction {
    fn operate(&self, s: &mut Vec<char>) {
        match *self {
            Instruction::SwapPos { pos1, pos2 } => {
                s.swap(pos1, pos2);
            }
            Instruction::SwapChars { ch1, ch2 } => {
                for c in s {
                    if *c == ch1 {
                        *c = ch2;
                    } else if *c == ch2 {
                        *c = ch1;
                    }
                }
            }
            Instruction::RotateLeft(n) => s.rotate_left(n),
            Instruction::RotateRight(n) => s.rotate_right(n),
            Instruction::RotatePos(ch) => {
                let i = s.iter().position(|&c| c == ch).unwrap();
                let shifts = (i + 1 + usize::from(i >= 4)) % s.len();
                s.rotate_right(shifts);
            }
            Instruction::Reverse { start, end } => s[start..=end].reverse(),
            Instruction::Move { src, dest } => {
                let ch = s.remove(src);
                s.insert(dest, ch);
            }
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Instruction, ParseError> {
        let mut parser = PullParser::new(s);
        match parser.scan_to(Token::Whitespace)? {
            "swap" => match parser.scan_to(Token::Whitespace)? {
                "position" => {
                    let pos1 = parser.parse_to::<usize, _>(Token::Whitespace)?;
                    parser.skip("with position ")?;
                    let pos2 = parser.parse_to::<usize, _>(Token::Eof)?;
                    Ok(Instruction::SwapPos { pos1, pos2 })
                }
                "letter" => {
                    let ch1 = parser.parse_to::<char, _>(Token::Whitespace)?;
                    parser.skip("with letter ")?;
                    let ch2 = parser.parse_to::<char, _>(Token::Eof)?;
                    Ok(Instruction::SwapChars { ch1, ch2 })
                }
                s => Err(ParseError::InvalidToken(s.to_owned())),
            },
            "rotate" => match parser.scan_to(Token::Whitespace)? {
                "left" => {
                    let n = parser.parse_to::<usize, _>(Token::Whitespace)?;
                    parser.skip("step")?;
                    if n != 1 {
                        parser.skip('s')?;
                    }
                    parser.eof()?;
                    Ok(Instruction::RotateLeft(n))
                }
                "right" => {
                    let n = parser.parse_to::<usize, _>(Token::Whitespace)?;
                    parser.skip("step")?;
                    if n != 1 {
                        parser.skip('s')?;
                    }
                    parser.eof()?;
                    Ok(Instruction::RotateRight(n))
                }
                "based" => {
                    parser.skip("on position of letter ")?;
                    let ch = parser.parse_to::<char, _>(Token::Eof)?;
                    Ok(Instruction::RotatePos(ch))
                }
                s => Err(ParseError::InvalidToken(s.to_owned())),
            },
            "reverse" => {
                parser.skip("positions ")?;
                let start = parser.parse_to::<usize, _>(Token::Whitespace)?;
                parser.skip("through ")?;
                let end = parser.parse_to::<usize, _>(Token::Eof)?;
                Ok(Instruction::Reverse { start, end })
            }
            "move" => {
                parser.skip("position ")?;
                let src = parser.parse_to::<usize, _>(Token::Whitespace)?;
                parser.skip("to position ")?;
                let dest = parser.parse_to::<usize, _>(Token::Eof)?;
                Ok(Instruction::Move { src, dest })
            }
            s => Err(ParseError::InvalidToken(s.to_owned())),
        }
    }
}

fn scramble(input: Input, start: &str) -> String {
    let mut s = start.chars().collect::<Vec<_>>();
    input
        .parse_lines::<Instruction>()
        .for_each(|instr| instr.operate(&mut s));
    String::from_iter(s)
}

fn solve(input: Input) -> String {
    scramble(input, "abcdefgh")
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
            "swap position 4 with position 0\n",
            "swap letter d with letter b\n",
            "reverse positions 0 through 4\n",
            "rotate left 1 step\n",
            "move position 1 to position 4\n",
            "move position 3 to position 0\n",
            "rotate based on position of letter b\n",
            "rotate based on position of letter d\n",
        ));
        assert_eq!(scramble(input, "abcde"), "decab");
    }
}
