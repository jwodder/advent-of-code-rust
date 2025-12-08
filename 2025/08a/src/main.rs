use adventutil::counter::Counter;
use adventutil::maxn::maxn;
use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{Input, unordered_pairs};
use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Junction {
    x: u32,
    y: u32,
    z: u32,
}

impl Junction {
    fn distance(self, other: Junction) -> f64 {
        let d = self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2);
        (d as f64).sqrt()
    }
}

impl std::str::FromStr for Junction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Junction, ParseError> {
        let mut parser = PullParser::new(s);
        let x = parser.parse_to::<u32, _>(',')?;
        let y = parser.parse_to::<u32, _>(',')?;
        let z = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(Junction { x, y, z })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CircuitWorld {
    junctions2circuits: HashMap<Junction, usize>,
    next_id: usize,
}

impl CircuitWorld {
    fn new() -> CircuitWorld {
        CircuitWorld {
            junctions2circuits: HashMap::new(),
            next_id: 0,
        }
    }

    fn connect(&mut self, a: Junction, b: Junction) {
        match (
            self.junctions2circuits.get(&a).copied(),
            self.junctions2circuits.get(&b).copied(),
        ) {
            (Some(ca), Some(cb)) if ca == cb => (),
            (Some(ca), Some(cb)) => {
                for p in &mut self.junctions2circuits {
                    if *p.1 == cb {
                        *p.1 = ca;
                    }
                }
            }
            (Some(ca), None) => {
                self.junctions2circuits.insert(b, ca);
            }
            (None, Some(cb)) => {
                self.junctions2circuits.insert(a, cb);
            }
            (None, None) => {
                let cid = self.next_id;
                self.next_id += 1;
                self.junctions2circuits.insert(a, cid);
                self.junctions2circuits.insert(b, cid);
            }
        }
    }

    fn largest(self) -> (u64, u64, u64) {
        let counter = Counter::from_iter(self.junctions2circuits.into_values());
        let max3 = maxn(3, counter.into_values());
        assert_eq!(max3.len(), 3);
        (max3[0], max3[1], max3[2])
    }
}

fn solve(input: Input, connections: usize) -> u64 {
    let junctions = input.parse_lines::<Junction>().collect::<Vec<_>>();
    let mut distances = BTreeMap::<OrderedFloat<f64>, Vec<(Junction, Junction)>>::new();
    for (&a, &b) in unordered_pairs(&junctions) {
        let dist = OrderedFloat(a.distance(b));
        distances.entry(dist).or_default().push((a, b));
    }
    let mut circuits = CircuitWorld::new();
    for (a, b) in distances.into_values().flatten().take(connections) {
        circuits.connect(a, b);
    }
    let (a, b, c) = circuits.largest();
    a * b * c
}

fn main() {
    println!("{}", solve(Input::from_env(), 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "162,817,812\n",
            "57,618,57\n",
            "906,360,560\n",
            "592,479,940\n",
            "352,342,300\n",
            "466,668,158\n",
            "542,29,236\n",
            "431,825,988\n",
            "739,650,466\n",
            "52,470,668\n",
            "216,146,977\n",
            "819,987,18\n",
            "117,168,530\n",
            "805,96,715\n",
            "346,949,466\n",
            "970,615,88\n",
            "941,993,340\n",
            "862,61,35\n",
            "984,92,344\n",
            "425,690,689\n",
        ));
        assert_eq!(solve(input, 10), 40);
    }
}
