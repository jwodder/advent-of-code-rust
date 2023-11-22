use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Number(u64),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

fn tokenize(s: &str) -> Tokenize<'_> {
    Tokenize {
        inner: s.chars().peekable(),
    }
}

struct Tokenize<'a> {
    inner: std::iter::Peekable<std::str::Chars<'a>>,
}

impl Iterator for Tokenize<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.inner.next()? {
                '+' => return Some(Token::Add),
                '*' => return Some(Token::Mul),
                '(' => return Some(Token::OpenParen),
                ')' => return Some(Token::CloseParen),
                c if c.is_ascii_whitespace() => continue,
                c if c.is_ascii_digit() => {
                    let mut digits = String::from(c);
                    while matches!(self.inner.peek(), Some(d) if d.is_ascii_digit()) {
                        digits.push(self.inner.next().unwrap());
                    }
                    return Some(Token::Number(digits.parse().unwrap()));
                }
                c => panic!("Unexpected character {c:?}"),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Frame {
    Empty,
    Number(u64),
    PartialAdd(u64),
    PartialMul(u64),
}

impl Frame {
    fn absorb(self, next: Token) -> Frame {
        match (self, next) {
            (Frame::Empty, Token::Number(n)) => Frame::Number(n),
            (Frame::Number(n), Token::Add) => Frame::PartialAdd(n),
            (Frame::Number(n), Token::Mul) => Frame::PartialMul(n),
            (Frame::PartialAdd(a), Token::Number(b)) => Frame::Number(a + b),
            (Frame::PartialMul(a), Token::Number(b)) => Frame::Number(a * b),
            (f, t) => panic!("Unexpected token {t:?} in stack state {f:?}"),
        }
    }
}

fn eval_expr(s: &str) -> u64 {
    let mut stack = vec![Frame::Empty];
    for t in tokenize(s) {
        match t {
            Token::OpenParen => stack.push(Frame::Empty),
            Token::CloseParen => match stack.pop() {
                Some(Frame::Number(n)) => {
                    let i = stack.len() - 1;
                    stack[i] = stack[i].absorb(Token::Number(n));
                }
                Some(f) => panic!("Parenthesized expression ended in state {f:?}"),
                None => panic!("No opening parenthesis for closing parenthesis"),
            },
            t => {
                let i = stack.len() - 1;
                stack[i] = stack[i].absorb(t);
            }
        }
    }
    match stack.len() {
        0 => unreachable!(),
        1 => match stack[0] {
            Frame::Number(n) => n,
            f => panic!("Expression ended in state {f:?}"),
        },
        _ => panic!("Unclosed parentheses"),
    }
}

fn solve(input: Input) -> u64 {
    input.lines().map(|s| eval_expr(&s)).sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1 + 2 * 3 + 4 * 5 + 6", 71)]
    #[case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[case("2 * 3 + (4 * 5)", 26)]
    #[case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)]
    #[case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240)]
    #[case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)]
    fn test_eval_expr(#[case] s: &str, #[case] value: u64) {
        assert_eq!(eval_expr(s), value);
    }
}
