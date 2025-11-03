use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{Input, dijkstra_length};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    floors: [BTreeSet<Item>; 4],
    elevator: usize,
}

impl State {
    fn end(&self) -> State {
        let all_items = self
            .floors
            .iter()
            .flatten()
            .cloned()
            .collect::<BTreeSet<_>>();
        State {
            floors: [BTreeSet::new(), BTreeSet::new(), BTreeSet::new(), all_items],
            elevator: 3,
        }
    }

    fn advancements(&self) -> Vec<State> {
        let mut next_states = Vec::new();
        for picked in pick_ones_and_twos(&self.floors[self.elevator]) {
            next_states.extend(self.move_up_with(&picked));
            next_states.extend(self.move_down_with(&picked));
        }
        next_states
    }

    fn move_up_with(&self, items: &[&Item]) -> Option<State> {
        if self.elevator >= 3 {
            return None;
        }
        let mut newstate = self.clone();
        for it in items {
            if !newstate.floors[newstate.elevator].remove(it) {
                return None;
            }
        }
        if !is_safe(&newstate.floors[newstate.elevator]) {
            return None;
        }
        newstate.elevator += 1;
        for it in items {
            newstate.floors[newstate.elevator].insert((*it).clone());
        }
        if !is_safe(&newstate.floors[newstate.elevator]) {
            return None;
        }
        Some(newstate)
    }

    fn move_down_with(&self, items: &[&Item]) -> Option<State> {
        if self.elevator == 0 {
            return None;
        }
        if self.floors[0..self.elevator].iter().all(BTreeSet::is_empty) {
            return None;
        }
        let mut newstate = self.clone();
        for it in items {
            if !newstate.floors[newstate.elevator].remove(it) {
                return None;
            }
        }
        if !is_safe(&newstate.floors[newstate.elevator]) {
            return None;
        }
        newstate.elevator -= 1;
        for it in items {
            newstate.floors[newstate.elevator].insert((*it).clone());
        }
        if !is_safe(&newstate.floors[newstate.elevator]) {
            return None;
        }
        Some(newstate)
    }
}

fn is_safe(set: &BTreeSet<Item>) -> bool {
    let mut microchips = BTreeSet::new();
    let mut generators = BTreeSet::new();
    for item in set {
        match item {
            Item::Microchip(element) => microchips.insert(element),
            Item::Generator(element) => generators.insert(element),
        };
    }
    microchips.is_subset(&generators) || generators.is_empty()
}

/// Return all one-element and two-element subsets of `set`, with the following
/// exceptions:
///
/// - If there are any pairs of a chip with its generator, return the first
///   pair only; any further pairs produce isomorphic (and thus redundant)
///   states.
fn pick_ones_and_twos(set: &BTreeSet<Item>) -> Vec<Vec<&Item>> {
    let items = set.iter().collect::<Vec<_>>();
    let mut picked = Vec::new();
    let mut got_pair = false;
    for i in 0..(items.len()) {
        picked.push(vec![items[i]]);
        for j in (i + 1)..(items.len()) {
            let it1 = items[i];
            let it2 = items[j];
            if it1.matches(it2) {
                if !got_pair {
                    picked.push(vec![it1, it2]);
                    got_pair = true;
                }
            } else {
                picked.push(vec![it1, it2]);
            }
        }
    }
    picked
}

impl std::str::FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<State, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("The first floor contains ")?;
        let first = parse_item_list(parser.scan_to('.')?)?;
        parser.skip(Token::Newline)?;
        parser.skip("The second floor contains ")?;
        let second = parse_item_list(parser.scan_to('.')?)?;
        parser.skip(Token::Newline)?;
        parser.skip("The third floor contains ")?;
        let third = parse_item_list(parser.scan_to('.')?)?;
        parser.skip(Token::Newline)?;
        parser.skip("The fourth floor contains nothing relevant.")?;
        //parser.skip(Token::Newline)?;
        parser.eof()?;
        Ok(State {
            floors: [first, second, third, BTreeSet::new()],
            elevator: 0,
        })
    }
}

fn parse_item_list(s: &str) -> Result<BTreeSet<Item>, ParseError> {
    if s.contains(',') {
        s.split(',')
            .map(|t| {
                let t = t.trim();
                let t = t.strip_prefix("and ").unwrap_or(t);
                t.parse::<Item>()
            })
            .collect()
    } else if let Some((pre, post)) = s.split_once(" and ") {
        Ok(BTreeSet::from([
            pre.parse::<Item>()?,
            post.parse::<Item>()?,
        ]))
    } else {
        Ok(BTreeSet::from([s.parse::<Item>()?]))
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Item {
    Microchip(String),
    Generator(String),
}

impl Item {
    fn matches(&self, other: &Item) -> bool {
        match (self, other) {
            (Item::Microchip(name1), Item::Generator(name2)) => name1 == name2,
            (Item::Generator(name1), Item::Microchip(name2)) => name1 == name2,
            _ => false,
        }
    }
}

impl std::str::FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Item, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("a ")?;
        let word = parser.scan_to(Token::Whitespace)?;
        if let Some(element) = word.strip_suffix("-compatible") {
            parser.skip("microchip")?;
            parser.eof()?;
            Ok(Item::Microchip(element.to_owned()))
        } else {
            parser.skip("generator")?;
            parser.eof()?;
            Ok(Item::Generator(word.to_owned()))
        }
    }
}

fn solve(input: Input) -> u32 {
    let start = input.parse::<State>();
    let end = start.end();
    dijkstra_length(
        start,
        |n| n == &end,
        |state| state.advancements().into_iter().map(|s| (s, 1)),
    )
    .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from(concat!(
            "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.\n",
            "The second floor contains a hydrogen generator.\n",
            "The third floor contains a lithium generator.\n",
            "The fourth floor contains nothing relevant.\n",
        ));
        assert_eq!(solve(input), 11);
    }
}
