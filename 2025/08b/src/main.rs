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
        let xdiff = self.x.abs_diff(other.x) as f64;
        let ydiff = self.y.abs_diff(other.y) as f64;
        let zdiff = self.z.abs_diff(other.z) as f64;
        (zdiff.mul_add(zdiff, xdiff.mul_add(xdiff, ydiff * ydiff))).sqrt()
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
    circuit_sizes: HashMap<usize, usize>,
    next_id: usize,
}

impl CircuitWorld {
    fn new() -> CircuitWorld {
        CircuitWorld {
            junctions2circuits: HashMap::new(),
            circuit_sizes: HashMap::new(),
            next_id: 0,
        }
    }

    // Returns the size of the resulting circuit
    fn connect(&mut self, a: Junction, b: Junction) -> usize {
        match (
            self.junctions2circuits.get(&a).copied(),
            self.junctions2circuits.get(&b).copied(),
        ) {
            (Some(ca), Some(cb)) if ca == cb => self.circuit_sizes.get(&ca).copied().unwrap(),
            (Some(ca), Some(cb)) => {
                let cb_size = self.circuit_sizes.get(&cb).copied().unwrap();
                let ca_size = self.circuit_sizes.get_mut(&ca).unwrap();
                *ca_size += cb_size;
                let r = *ca_size;
                for p in &mut self.junctions2circuits {
                    if *p.1 == cb {
                        *p.1 = ca;
                    }
                }
                r
            }
            (Some(ca), None) => {
                self.junctions2circuits.insert(b, ca);
                let sz = self.circuit_sizes.get_mut(&ca).unwrap();
                *sz += 1;
                *sz
            }
            (None, Some(cb)) => {
                self.junctions2circuits.insert(a, cb);
                let sz = self.circuit_sizes.get_mut(&cb).unwrap();
                *sz += 1;
                *sz
            }
            (None, None) => {
                let cid = self.next_id;
                self.next_id += 1;
                self.junctions2circuits.insert(a, cid);
                self.junctions2circuits.insert(b, cid);
                self.circuit_sizes.insert(cid, 2);
                1
            }
        }
    }
}

fn solve(input: Input) -> u32 {
    let junctions = input.parse_lines::<Junction>().collect::<Vec<_>>();
    let mut distances = BTreeMap::<OrderedFloat<f64>, Vec<(Junction, Junction)>>::new();
    for (&a, &b) in unordered_pairs(&junctions) {
        let dist = OrderedFloat(a.distance(b));
        distances.entry(dist).or_default().push((a, b));
    }
    let mut circuits = CircuitWorld::new();
    for (a, b) in distances.into_values().flatten() {
        if circuits.connect(a, b) == junctions.len() {
            return a.x * b.x;
        }
    }
    panic!("No mono-circuit formed");
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
        assert_eq!(solve(input), 25272);
    }
}
