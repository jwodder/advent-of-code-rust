use adventutil::Input;
use adventutil::gridgeom::{Point, Vector};
use adventutil::maxtracker::MaxTracker;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct World(HashMap<Point, HashSet<Point>>);

impl World {
    fn add_adjacency(&mut self, p1: Point, p2: Point) {
        self.0.entry(p1).or_default().insert(p2);
        self.0.entry(p2).or_default().insert(p1);
    }

    fn eccentricity(&self) -> u32 {
        let mut seen = HashSet::new();
        let mut nodes = vec![Point::ORIGIN];
        let mut dist = 0;
        let mut tracker = MaxTracker::new();
        while !nodes.is_empty() {
            let mut nodes2 = Vec::new();
            for current in nodes {
                let mut any_steps = false;
                for &p in &self.0[&current] {
                    if seen.insert(p) {
                        any_steps = true;
                        nodes2.push(p);
                    }
                }
                if !any_steps {
                    tracker.add(dist);
                }
            }
            nodes = nodes2;
            dist += 1;
        }
        tracker.get().unwrap()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Exploration {
    current: HashSet<Point>,
    branches: Vec<Branch>,
    world: World,
}

impl Exploration {
    fn new() -> Exploration {
        Exploration {
            current: HashSet::from([Point::ORIGIN]),
            branches: Vec::new(),
            world: World::default(),
        }
    }

    fn domove(&mut self, dir: Vector) {
        let mut current2 = HashSet::with_capacity(self.current.len());
        for p in std::mem::take(&mut self.current) {
            let p2 = p + dir;
            self.world.add_adjacency(p, p2);
            current2.insert(p2);
        }
        self.current = current2;
    }

    fn start_branch(&mut self) {
        self.branches.push(Branch {
            start: self.current.clone(),
            completed: HashSet::new(),
        });
    }

    fn alternate(&mut self) {
        let Some(br) = self.branches.last_mut() else {
            panic!("alternate() called while not branching");
        };
        let complete = std::mem::replace(&mut self.current, br.start.clone());
        br.completed.extend(complete);
    }

    fn end_branch(&mut self) {
        let Some(br) = self.branches.pop() else {
            panic!("end_branch() called while not branching");
        };
        self.current.extend(br.completed);
    }

    fn end(self) -> World {
        assert!(self.branches.is_empty(), "end() called while branching");
        self.world
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Branch {
    start: HashSet<Point>,
    completed: HashSet<Point>,
}

fn solve(input: Input) -> u32 {
    let mut explore = Exploration::new();
    for ch in input.read().trim().chars() {
        match ch {
            '^' => (),
            'N' => explore.domove(Vector::NORTH),
            'S' => explore.domove(Vector::SOUTH),
            'E' => explore.domove(Vector::EAST),
            'W' => explore.domove(Vector::WEST),
            '(' => explore.start_branch(),
            '|' => explore.alternate(),
            ')' => explore.end_branch(),
            '$' => {
                let world = explore.end();
                return world.eccentricity();
            }
            _ => panic!("Unexpcted character {ch:?} in input"),
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("^WNE$", 3)]
    #[case("^ENWWW(NEEE|SSE(EE|N))$", 10)]
    #[case("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18)]
    #[case("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23)]
    #[case(
        "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
        31
    )]
    fn example1(#[case] regex: &'static str, #[case] answer: u32) {
        let input = Input::from(regex);
        assert_eq!(solve(input), answer);
    }
}
