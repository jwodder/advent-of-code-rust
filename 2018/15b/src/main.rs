use adventutil::grid::{Coords, Direction, Grid, GridBounds, ParseGridError};
use adventutil::Input;
use std::cell::RefCell;
use std::collections::{btree_map::Entry, BTreeMap, HashSet};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Battlefield {
    units: Vec<Rc<RefCell<Unit>>>,
    bounds: GridBounds,
    open: HashSet<Coords>,
    elf_power: u32,
}

impl Battlefield {
    fn round(&mut self) -> RoundOutcome {
        let mut unit_order = self.units.clone();
        unit_order.sort_unstable_by_key(|u| u.borrow().pos());
        for unit in unit_order {
            if unit.borrow().health() > 0 {
                let species = unit.borrow().species();
                self.move_unit(Rc::clone(&unit));
                match self.attack(unit) {
                    AttackOutcome::Ongoing => (),
                    AttackOutcome::Kill => {
                        if species == Species::Goblin {
                            return RoundOutcome::ElfDeath;
                        }
                    }
                    AttackOutcome::Victory => {
                        assert_eq!(species, Species::Elf);
                        return RoundOutcome::ElfVictory;
                    }
                }
            }
        }
        RoundOutcome::Ongoing
    }

    fn remaining_hp(&self) -> u32 {
        self.units.iter().map(|u| u.borrow().health()).sum()
    }

    fn move_unit(&mut self, unit: Rc<RefCell<Unit>>) {
        let start = unit.borrow().pos();
        let enemy = unit.borrow().species.enemy();
        let mut in_range = HashSet::new();
        for u in self.units.clone() {
            let u = u.borrow();
            if u.species() == enemy && u.health() > 0 {
                for d in Direction::cardinals() {
                    if let Some(c) = self.bounds.move_in(u.pos(), d) {
                        if self.open.contains(&c) {
                            in_range.insert(c);
                        } else if c == start {
                            return;
                        }
                    }
                }
            }
        }
        if in_range.is_empty() {
            return;
        }

        let mut visited = HashSet::new();
        let mut pos2path = BTreeMap::from([(start, Vec::new())]);
        let mut shortest = BTreeMap::<Coords, Vec<Coords>>::new();
        while shortest.is_empty() {
            if pos2path.is_empty() {
                // No paths to any in_range coords
                return;
            }
            let mut pos2path2 = BTreeMap::<Coords, Vec<Coords>>::new();
            for (c, path) in pos2path {
                for c2 in Direction::cardinals()
                    .filter_map(|d| self.bounds.move_in(c, d))
                    .filter(|c2| self.open.contains(c2) && !visited.contains(c2))
                {
                    let mut newpath = path.clone();
                    newpath.push(c2);
                    let map = if in_range.contains(&c2) {
                        &mut shortest
                    } else {
                        &mut pos2path2
                    };
                    match map.entry(c2) {
                        Entry::Vacant(e) => {
                            e.insert(newpath);
                        }
                        Entry::Occupied(mut e) => {
                            if newpath[0] < e.get()[0] {
                                e.insert(newpath);
                            }
                        }
                    }
                }
                visited.insert(c);
            }
            pos2path = pos2path2;
        }
        let next_pos = shortest.pop_first().unwrap().1[0];

        self.open.insert(start);
        self.open.remove(&next_pos);
        unit.borrow_mut().pos = next_pos;
    }

    fn attack(&mut self, unit: Rc<RefCell<Unit>>) -> AttackOutcome {
        let adjacent_coords = Direction::cardinals()
            .filter_map(|d| self.bounds.move_in(unit.borrow().pos, d))
            .collect::<HashSet<_>>();
        let enemy = unit.borrow().species.enemy();
        let mut adj_enemies = Vec::new();
        let mut any_enemies = false;
        for u in self.units.clone() {
            if u.borrow().species() == enemy && u.borrow().health() > 0 {
                any_enemies = true;
                if adjacent_coords.contains(&u.borrow().pos()) {
                    adj_enemies.push(u);
                }
            }
        }
        if !any_enemies {
            return AttackOutcome::Victory;
        }
        if let Some(target_hp) = adj_enemies.iter().map(|u| u.borrow().health()).min() {
            let target = adj_enemies
                .into_iter()
                .filter(|u| u.borrow().health() == target_hp)
                .min_by_key(|u| u.borrow().pos())
                .unwrap();
            let mut t = target.borrow_mut();
            t.health = t.health.saturating_sub(if enemy == Species::Goblin {
                self.elf_power
            } else {
                3
            });
            if t.health == 0 {
                self.open.insert(t.pos());
                return AttackOutcome::Kill;
            }
        }
        AttackOutcome::Ongoing
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
            elf_power: 3,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RoundOutcome {
    Ongoing,
    ElfDeath,
    ElfVictory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttackOutcome {
    Ongoing,
    Kill,
    Victory,
}

fn solve(input: Input) -> u32 {
    let battle = input.parse::<Battlefield>();
    'powerloop: for power in 4.. {
        let mut enhbattle = battle.clone();
        enhbattle.elf_power = power;
        let mut rounds = 0;
        loop {
            match enhbattle.round() {
                RoundOutcome::Ongoing => (),
                RoundOutcome::ElfDeath => continue 'powerloop,
                RoundOutcome::ElfVictory => return rounds * battle.remaining_hp(),
            }
            rounds += 1;
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
        assert_eq!(solve(input), 4988);
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
        assert_eq!(solve(input), 31284);
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
        assert_eq!(solve(input), 3478);
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
        assert_eq!(solve(input), 6474);
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
        assert_eq!(solve(input), 1140);
    }
}
