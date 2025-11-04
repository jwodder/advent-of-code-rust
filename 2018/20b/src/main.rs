use adventutil::gridgeom::{Point, Vector};
use adventutil::{DistanceMap, Input};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct World(HashMap<Point, HashSet<Point>>);

impl World {
    fn add_adjacency(&mut self, p1: Point, p2: Point) {
        self.0.entry(p1).or_default().insert(p2);
        self.0.entry(p2).or_default().insert(p1);
    }

    fn thousand_eccentricities(&self) -> usize {
        let mut visited = HashSet::new();
        let mut distances = DistanceMap::new();
        distances.insert(Point::ORIGIN, 0);
        let mut qty = 0;
        loop {
            let Some((current, dist)) = distances.pop_nearest() else {
                return qty;
            };
            if dist >= 1000 {
                qty += 1;
            }
            for &p in &self.0[&current] {
                if !visited.contains(&p) {
                    distances.insert(p, dist + 1);
                }
            }
            visited.insert(current);
        }
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

fn solve(input: Input) -> usize {
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
                return world.thousand_eccentricities();
            }
            _ => panic!("Unexpcted character {ch:?} in input"),
        }
    }
    unreachable!()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
