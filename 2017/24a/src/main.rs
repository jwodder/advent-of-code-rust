use adventutil::Input;
use adventutil::maxtracker::MaxTracker;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Component {
    port1: u32,
    port2: u32,
}

impl Component {
    fn strength(self) -> u32 {
        self.port1 + self.port2
    }

    fn across_from(self, p: u32) -> Option<u32> {
        if p == self.port1 {
            Some(self.port2)
        } else if p == self.port2 {
            Some(self.port1)
        } else {
            None
        }
    }
}

impl std::str::FromStr for Component {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Component, ParseError> {
        let mut parser = PullParser::new(s);
        let port1 = parser.parse_to::<u32, _>('/')?;
        let port2 = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(Component { port1, port2 })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bridge {
    port: u32,
    strength: u32,
    unused: HashSet<Component>,
}

impl Bridge {
    fn advance(self) -> impl Iterator<Item = Bridge> {
        let unused0 = self.unused.clone();
        self.unused.into_iter().filter_map(move |c| {
            let port = c.across_from(self.port)?;
            let strength = self.strength + c.strength();
            let mut unused = unused0.clone();
            unused.remove(&c);
            Some(Bridge {
                port,
                strength,
                unused,
            })
        })
    }
}

fn solve(input: Input) -> u32 {
    let components = input.parse_lines::<Component>().collect::<HashSet<_>>();
    let mut bridges = vec![Bridge {
        port: 0,
        strength: 0,
        unused: components,
    }];
    let mut tracker = MaxTracker::new();
    while !bridges.is_empty() {
        let mut bridges2 = Vec::new();
        for b in bridges {
            let strength = b.strength;
            let prelen = bridges2.len();
            bridges2.extend(b.advance());
            if bridges2.len() == prelen {
                tracker.add(strength);
            }
        }
        bridges = bridges2;
    }
    tracker.get().unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10\n");
        assert_eq!(solve(input), 31);
    }
}
