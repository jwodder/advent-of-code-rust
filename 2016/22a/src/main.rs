use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

impl std::str::FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Node, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("/dev/grid/node-x")?;
        let x = parser.parse_to::<usize, _>('-')?;
        parser.skip('y')?;
        let y = parser.parse_to::<usize, _>(Token::Whitespace)?;
        let size = parser.parse_to::<usize, _>('T')?;
        parser.skip(Token::Whitespace)?;
        let used = parser.parse_to::<usize, _>('T')?;
        parser.skip(Token::Whitespace)?;
        let avail = parser.parse_to::<usize, _>('T')?;
        parser.skip(Token::Whitespace)?;
        let _use_pct = parser.parse_to::<usize, _>('%')?;
        parser.eof()?;
        Ok(Node {
            x,
            y,
            size,
            used,
            avail,
        })
    }
}

fn solve(input: Input) -> usize {
    let nodes = input
        .lines()
        .skip(2)
        .map(|ln| ln.parse::<Node>().unwrap())
        .collect::<Vec<_>>();
    let mut qty = 0;
    for a in &nodes {
        for b in &nodes {
            if a != b && a.used != 0 && a.used <= b.avail {
                qty += 1;
            }
        }
    }
    qty
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
