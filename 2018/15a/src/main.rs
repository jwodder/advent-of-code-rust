use adventutil::grid::{Coords, Direction, Grid, GridBounds, ParseGridError};
use adventutil::Input;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Battlefield {
    units: HashMap<usize, Unit>,
    bounds: GridBounds,
    open: HashSet<Coords>,
}

impl Battlefield {
    // Returns `true` if the round ended with one side's victory
    fn round(&mut self) -> bool {
        let mut unit_order = self
            .units
            .iter()
            .map(|(&i, unit)| (i, unit.clone()))
            .collect::<Vec<_>>();
        unit_order.sort_unstable_by_key(|p| p.1.pos());
        for (id, mut unit) in unit_order {
            if self.units.contains_key(&id) {
                unit = self.move_unit(id, unit);
                if !self.attack(unit) {
                    return true;
                }
            }
        }
        false
    }

    fn remaining_hp(&self) -> u32 {
        self.units.values().map(Unit::health).sum()
    }

    fn move_unit(&mut self, unit_id: usize, mut unit: Unit) -> Unit {
        let enemy = unit.species.enemy();
        let mut in_range = HashSet::new();
        for u in self.units.values() {
            if u.species() == enemy {
                for d in Direction::cardinals() {
                    if let Some(c) = self.bounds.move_in(u.pos(), d) {
                        if self.open.contains(&c) {
                            in_range.insert(c);
                        }
                    }
                }
            }
        }
        if in_range.contains(&unit.pos()) {
            return unit;
        }

        let mut visited = HashSet::new();
        let mut pos2paths = HashMap::from([(unit.pos(), vec![Vec::new()])]);
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
        }
        let (_, paths) = shortest.pop_first().unwrap();
        let next_pos = paths.into_iter().map(|pth| pth[0]).min().unwrap();

        self.open.insert(unit.pos);
        self.open.remove(&next_pos);
        unit.pos = next_pos;
        self.units.insert(unit_id, unit.clone());
        unit
    }

    // Returns `false` if there were no enemies anywhere on the map to attack
    fn attack(&mut self, unit: Unit) -> bool {
        let adjacent_coords = Direction::cardinals()
            .filter_map(|d| self.bounds.move_in(unit.pos, d))
            .collect::<HashSet<_>>();
        let enemy = unit.species.enemy();
        let mut adj_enemies = Vec::new();
        let mut any_enemies = false;
        for (&id, u) in &self.units {
            if u.species() == enemy {
                any_enemies = true;
                if adjacent_coords.contains(&u.pos()) {
                    adj_enemies.push((id, u.clone()));
                }
            }
        }
        if !any_enemies {
            return false;
        }
        if let Some(target_hp) = adj_enemies.iter().map(|(_, u)| u.health()).min() {
            let (target_id, mut target) = adj_enemies
                .into_iter()
                .filter(|p| p.1.health() == target_hp)
                .min_by_key(|p| p.1.pos())
                .unwrap();
            target.health = target.health.saturating_sub(3);
            if target.health == 0 {
                self.units.remove(&target_id);
                self.open.insert(target.pos());
            } else {
                self.units.insert(target_id, target);
            }
        }
        true
    }
}

impl std::str::FromStr for Battlefield {
    type Err = ParseGridError<<char as std::str::FromStr>::Err>;

    fn from_str(s: &str) -> Result<Battlefield, Self::Err> {
        let grid = s.parse::<Grid<char>>()?;
        let mut i = 0;
        let mut units = HashMap::new();
        let bounds = grid.bounds();
        let mut open = HashSet::new();
        for (pos, &ch) in &grid {
            match ch {
                '.' => {
                    open.insert(pos);
                }
                '#' => (),
                'G' => {
                    units.insert(
                        i,
                        Unit {
                            pos,
                            health: 200,
                            species: Species::Goblin,
                        },
                    );
                    i += 1;
                }
                'E' => {
                    units.insert(
                        i,
                        Unit {
                            pos,
                            health: 200,
                            species: Species::Elf,
                        },
                    );
                    i += 1;
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
