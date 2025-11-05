use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{Input, unit_dijkstra_length};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    floors: [Items; 4],
    elevator: usize,
}

impl State {
    fn is_end(&self) -> bool {
        (0..3).all(|i| self.floors[i].is_empty()) && self.elevator == 3
    }

    fn advancements(&self) -> Vec<State> {
        let mut next_states = Vec::new();
        for picked in self.floors[self.elevator].pick_ones_and_twos() {
            next_states.extend(self.move_up_with(&picked));
            next_states.extend(self.move_down_with(&picked));
        }
        next_states
    }

    fn move_up_with(&self, items: &Items) -> Option<State> {
        if self.elevator >= 3 {
            return None;
        }
        let mut newstate = self.clone();
        newstate.floors[newstate.elevator].remove(items);
        assert!(newstate.floors[newstate.elevator].is_safe());
        newstate.elevator += 1;
        newstate.floors[newstate.elevator].extend(items.clone());
        if !newstate.floors[newstate.elevator].is_safe() {
            return None;
        }
        Some(newstate)
    }

    fn move_down_with(&self, items: &Items) -> Option<State> {
        if self.elevator == 0 {
            return None;
        }
        if self.floors[0..self.elevator].iter().all(Items::is_empty) {
            return None;
        }
        let mut newstate = self.clone();
        newstate.floors[newstate.elevator].remove(items);
        assert!(newstate.floors[newstate.elevator].is_safe());
        newstate.elevator -= 1;
        newstate.floors[newstate.elevator].extend(items.clone());
        if !newstate.floors[newstate.elevator].is_safe() {
            return None;
        }
        Some(newstate)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Items {
    microchips: BTreeSet<String>,
    generators: BTreeSet<String>,
}

impl Items {
    fn is_empty(&self) -> bool {
        self.microchips.is_empty() && self.generators.is_empty()
    }

    fn is_safe(&self) -> bool {
        self.microchips.is_subset(&self.generators) || self.generators.is_empty()
    }

    fn remove(&mut self, other: &Items) {
        for chip in &other.microchips {
            self.microchips.remove(chip);
        }
        for generator in &other.generators {
            self.generators.remove(generator);
        }
    }

    fn extend(&mut self, other: Items) {
        self.microchips.extend(other.microchips);
        self.generators.extend(other.generators);
    }

    /// Return all one-element and two-element subsets of `self`, with the
    /// following exceptions:
    ///
    /// - For safety reasons, if `self` contains both a chip and its generator,
    ///   and there are other generators, don't take the first generator
    ///   without the chip; if you're taking the generator, also take either
    ///   the chip or, if there is exactly one other generator, the other
    ///   generator.
    ///
    /// - As an optimization, if `self` contains any pairs of a chip with its
    ///   generator, return the first pair only; any further pairs produce
    ///   isomorphic (and thus redundant) states.
    ///
    /// This simplifies to the following:
    ///
    /// - All one- & two-element sets of microchips
    ///
    /// - All one- & two-element sets of generators that do not have
    ///   corresponding microchips
    ///
    /// - If there is only one generator, that generator (unless it lacks a
    ///   corresponding microchip, in which case it would be a duplicate)
    ///
    /// - If there are only two generators, both generators (unless they both
    ///   lack corresponding microchips, in which case this would be a
    ///   duplicate)
    ///
    /// - Exactly one matching microchip-generator pair, if there are any such
    ///   pairs
    fn pick_ones_and_twos(&self) -> Vec<Items> {
        let mut picked = Vec::new();
        let chips = self.microchips.iter().collect::<Vec<_>>();
        for microchips in pick_ones_and_twos(&chips) {
            picked.push(Items {
                microchips,
                generators: BTreeSet::new(),
            });
        }
        let unmatched_gens = self
            .generators
            .difference(&self.microchips)
            .collect::<Vec<_>>();
        for generators in pick_ones_and_twos(&unmatched_gens) {
            picked.push(Items {
                microchips: BTreeSet::new(),
                generators,
            });
        }
        if matches!(self.generators.len(), 1 | 2) && !self.generators.is_subset(&self.microchips) {
            picked.push(Items {
                microchips: BTreeSet::new(),
                generators: self.generators.clone(),
            });
        }
        if let Some(name) = self.microchips.intersection(&self.generators).next() {
            picked.push(Items {
                microchips: [name.clone()].into(),
                generators: [name.clone()].into(),
            });
        }
        picked
    }
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
            floors: [first, second, third, Items::default()],
            elevator: 0,
        })
    }
}

fn parse_item_list(s: &str) -> Result<Items, ParseError> {
    let unsorted = if s.contains(',') {
        s.split(',')
            .map(|t| {
                let t = t.trim();
                let t = t.strip_prefix("and ").unwrap_or(t);
                t.parse::<Item>()
            })
            .collect::<Result<Vec<_>, _>>()?
    } else if let Some((pre, post)) = s.split_once(" and ") {
        vec![pre.parse::<Item>()?, post.parse::<Item>()?]
    } else {
        vec![s.parse::<Item>()?]
    };
    let mut items = Items::default();
    for it in unsorted {
        match it {
            Item::Microchip(name) => items.microchips.insert(name),
            Item::Generator(name) => items.generators.insert(name),
        };
    }
    Ok(items)
}

fn pick_ones_and_twos(items: &[&String]) -> Vec<BTreeSet<String>> {
    let mut picked = Vec::new();
    for (i, &it1) in items.iter().enumerate() {
        picked.push([it1.clone()].into());
        for &it2 in items.iter().skip(i + 1) {
            picked.push([it1.clone(), it2.clone()].into());
        }
    }
    picked
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Item {
    Microchip(String),
    Generator(String),
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
    unit_dijkstra_length(start, State::is_end, State::advancements).unwrap()
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
