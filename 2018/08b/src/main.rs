use adventutil::Input;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    #[allow(unused)]
    children: Vec<Node>,
    #[allow(unused)]
    metadata: Vec<usize>,
    value: usize,
}

impl Node {
    fn new(children: Vec<Node>, metadata: Vec<usize>) -> Node {
        let value = if children.is_empty() {
            metadata.iter().copied().sum()
        } else {
            metadata
                .iter()
                .filter(|&&i| i != 0)
                .map(|&i| match children.get(i - 1) {
                    Some(n) => n.value,
                    None => 0,
                })
                .sum()
        };
        Node {
            children,
            metadata,
            value,
        }
    }

    fn parse_tree(s: String) -> Result<Node, ParseError> {
        let mut stream = NumberStream::new(&s);
        let node = Node::parse_node(&mut stream)?;
        stream.at_end()?;
        Ok(node)
    }

    fn parse_node(stream: &mut NumberStream<'_>) -> Result<Node, ParseError> {
        let child_qty = stream.get()?;
        let meta_qty = stream.get()?;
        let mut children = Vec::with_capacity(child_qty);
        for _ in 0..child_qty {
            children.push(Node::parse_node(stream)?);
        }
        let mut metadata = Vec::with_capacity(meta_qty);
        for _ in 0..meta_qty {
            metadata.push(stream.get()?);
        }
        Ok(Node::new(children, metadata))
    }
}

#[derive(Clone, Debug)]
struct NumberStream<'a> {
    inner: std::str::SplitWhitespace<'a>,
}

impl<'a> NumberStream<'a> {
    fn new(s: &'a str) -> Self {
        NumberStream {
            inner: s.split_whitespace(),
        }
    }

    fn get(&mut self) -> Result<usize, ParseError> {
        Ok(self
            .inner
            .next()
            .ok_or(ParseError::Short)?
            .parse::<usize>()?)
    }

    fn at_end(&mut self) -> Result<(), ParseError> {
        if self.inner.next().is_none() {
            Ok(())
        } else {
            Err(ParseError::Trailing)
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("input had too few components")]
    Short,
    #[error("input had trailing components")]
    Trailing,
    #[error("invalid integer: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
}

fn solve(input: Input) -> usize {
    Node::parse_tree(input.read()).unwrap().value
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(solve(input), 66);
    }
}
