use adventutil::grid::{Coords, Direction, Grid, GridBounds, ParseGridError};
use adventutil::Input;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Battlefield {
    units: Vec<Rc<RefCell<Unit>>>,
    bounds: GridBounds,
    open: HashSet<Coords>,
}

impl Battlefield {
    // Returns `true` if the round ended with one side's victory
    fn round(&mut self) -> bool {
        let mut unit_order = self.units.clone();
        unit_order.sort_unstable_by_key(|u| u.borrow().pos());
        for unit in unit_order {
            if unit.borrow().health() > 0 {
                self.move_unit(Rc::clone(&unit));
                if !self.attack(unit) {
                    return true;
                }
            }
        }
        false
    }

    fn remaining_hp(&self) -> u32 {
        self.units.iter().map(|u| u.borrow().health()).sum()
    }

    fn move_unit(&mut self, unit: Rc<RefCell<Unit>>) {
        let enemy = unit.borrow().species.enemy();
        let mut in_range = HashSet::new();
        for u in self.units.clone() {
            if u.borrow().species() == enemy {
                for d in Direction::cardinals() {
                    if let Some(c) = self.bounds.move_in(u.borrow().pos(), d) {
                        if self.open.contains(&c) {
                            in_range.insert(c);
                        }
                    }
                }
            }
        }
        let start = unit.borrow().pos();
        if in_range.is_empty() || in_range.contains(&start) {
            return;
        }

        let mut visited = HashSet::new();
        let mut pos2paths = HashMap::from([(start, vec![Vec::new()])]);
        let mut shortest = BTreeMap::<Coords, Vec<Vec<Coords>>>::new();
        while shortest.is_empty() {
            let mut pos2paths2 = HashMap::<Coords, Vec<Vec<Coords>>>::new();
            for (c, paths) in pos2paths {
                for c2 in Direction::cardinals()
                    .filter_map(|d| self.bounds.move_in(c, d))
                    .filter(|c2| self.open.contains(c2) && !visited.contains(c2))
                {
                    let newpaths = paths.iter().cloned().map(|mut pth| {
                        pth.push(c2);
                        pth
                    });
                    if in_range.contains(&c2) {
                        shortest.entry(c2).or_default().extend(newpaths);
                    } else {
                        pos2paths2.entry(c2).or_default().extend(newpaths);
                    }
                }
                visited.insert(c);
            }
            pos2paths = pos2paths2;
            if pos2paths.is_empty() {
                // No paths to any in_range coords
                return;
            }
        }
        let (_, paths) = shortest.pop_first().unwrap();
        let next_pos = paths.into_iter().map(|pth| pth[0]).min().unwrap();

        self.open.insert(start);
        self.open.remove(&next_pos);
        unit.borrow_mut().pos = next_pos;
    }

    // Returns `false` if there were no enemies anywhere on the map to attack
    fn attack(&mut self, unit: Rc<RefCell<Unit>>) -> bool {
        let adjacent_coords = Direction::cardinals()
            .filter_map(|d| self.bounds.move_in(unit.borrow().pos, d))
            .collect::<HashSet<_>>();
        let enemy = unit.borrow().species.enemy();
        let mut adj_enemies = Vec::new();
        let mut any_enemies = false;
        for u in self.units.clone() {
            if u.borrow().species() == enemy {
                any_enemies = true;
                if adjacent_coords.contains(&u.borrow().pos()) {
                    adj_enemies.push(u);
                }
            }
        }
        if !any_enemies {
            return false;
        }
        if let Some(target_hp) = adj_enemies.iter().map(|u| u.borrow().health()).min() {
            let target = adj_enemies
                .into_iter()
                .filter(|u| u.borrow().health() == target_hp)
                .min_by_key(|u| u.borrow().pos())
                .unwrap();
            let mut t = target.borrow_mut();
            t.health = t.health.saturating_sub(3);
            if t.health == 0 {
                self.open.insert(t.pos());
            }
        }
        true
    }
}

impl std::str::FromStr for Battlefield {
    type Err = ParseGridError<<char as std::str::FromStr>::Err>;

    fn from_str(s: &str) -> Result<Battlefield, Self::Err> {
        let grid = s.parse::<Grid<char>>()?;
        let mut units = Vec::new();
        let bounds = grid.bounds();
        let mut open = HashSet::new();
        for (pos, &ch) in &grid {
            match ch {
                '.' => {
                    open.insert(pos);
                }
                '#' => (),
                'G' => {
                    units.push(Rc::new(RefCell::new(Unit {
                        pos,
                        health: 200,
                        species: Species::Goblin,
                    })));
                }
                'E' => {
                    units.push(Rc::new(RefCell::new(Unit {
                        pos,
                        health: 200,
                        species: Species::Elf,
                    })));
                }
                _ => unreachable!(), // TODO: Use a real error
            }
        }
        Ok(Battlefield {
            units,
            bounds,
            open,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Unit {
    pos: Coords,
    health: u32,
    species: Species,
}

impl Unit {
    fn pos(&self) -> Coords {
        self.pos
    }

    fn health(&self) -> u32 {
        self.health
    }

    fn species(&self) -> Species {
        self.species
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Species {
    Goblin,
    Elf,
}

impl Species {
    fn enemy(self) -> Species {
        match self {
            Species::Goblin => Species::Elf,
            Species::Elf => Species::Goblin,
        }
    }
}

fn solve(input: Input) -> u32 {
    let mut battle = input.parse::<Battlefield>();
    let mut turn = 0;
    loop {
        if battle.round() {
            break;
        }
        turn += 1;
    }
    turn * battle.remaining_hp()
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
            "#######\n",
            "#.G...#\n",
            "#...EG#\n",
            "#.#.#G#\n",
            "#..G#E#\n",
            "#.....#\n",
            "#######\n",
        ));
        assert_eq!(solve(input), 27730);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "#######\n",
            "#G..#E#\n",
            "#E#E.E#\n",
            "#G.##.#\n",
            "#...#E#\n",
            "#...E.#\n",
            "#######\n",
        ));
        assert_eq!(solve(input), 36334);
    }

    #[test]
    fn example3() {
        let input = Input::from(concat!(
            "#######\n",
            "#E..EG#\n",
            "#.#G.E#\n",
            "#E.##E#\n",
            "#G..#.#\n",
            "#..E#.#\n",
            "#######\n",
        ));
        assert_eq!(solve(input), 39514);
    }

    #[test]
    fn example4() {
        let input = Input::from(concat!(
            "#######\n",
            "#E.G#.#\n",
            "#.#G..#\n",
            "#G.#.G#\n",
            "#G..#.#\n",
            "#...E.#\n",
            "#######\n",
        ));
        assert_eq!(solve(input), 27755);
    }

    #[test]
    fn example5() {
        let input = Input::from(concat!(
            "#######\n",
            "#.E...#\n",
            "#.#..G#\n",
            "#.###.#\n",
            "#E#G#G#\n",
            "#...#G#\n",
            "#######\n",
        ));
        assert_eq!(solve(input), 28944);
    }

    #[test]
    fn example6() {
        let input = Input::from(concat!(
            "#########\n",
            "#G......#\n",
            "#.E.#...#\n",
            "#..##..G#\n",
            "#...##..#\n",
            "#...#...#\n",
            "#.G...G.#\n",
            "#.....G.#\n",
            "#########\n",
        ));
        assert_eq!(solve(input), 18740);
    }
}
