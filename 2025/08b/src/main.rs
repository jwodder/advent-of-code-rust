use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{Input, unordered_pairs};
use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

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
struct CircuitWorld(HashMap<Junction, Rc<RefCell<HashSet<Junction>>>>);

impl CircuitWorld {
    fn new() -> CircuitWorld {
        CircuitWorld(HashMap::new())
    }

    // Returns the size of the resulting circuit
    fn connect(&mut self, a: Junction, b: Junction) -> usize {
        match (self.0.get(&a), self.0.get(&b)) {
            (Some(ca), Some(cb)) if Rc::ptr_eq(ca, cb) => ca.borrow().len(),
            (Some(ca), Some(cb)) => {
                let mut cam = ca.borrow_mut();
                let mut updates = HashMap::new();
                for &j in cb.borrow().iter() {
                    cam.insert(j);
                    updates.insert(j, Rc::clone(ca));
                }
                let r = cam.len();
                drop(cam);
                self.0.extend(updates);
                r
            }
            (Some(ca), None) => {
                ca.borrow_mut().insert(b);
                let r = ca.borrow().len();
                self.0.insert(b, Rc::clone(ca));
                r
            }
            (None, Some(cb)) => {
                cb.borrow_mut().insert(a);
                let r = cb.borrow().len();
                self.0.insert(a, Rc::clone(cb));
                r
            }
            (None, None) => {
                let c = Rc::new(RefCell::new(HashSet::from([a, b])));
                self.0.insert(a, Rc::clone(&c));
                self.0.insert(b, c);
                2
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
