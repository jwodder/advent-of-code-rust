use adventutil::grid::{Coords, Grid};
use adventutil::Input;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::f64::consts::{FRAC_PI_2, TAU};

fn solve(input: Input) -> usize {
    let grid = input.parse::<Grid<char>>().map(|c| c == '#');
    let asteroids = grid
        .enumerate()
        .filter_map(|(coords, &b)| b.then_some(coords))
        .collect::<HashSet<_>>();
    let mut rays = asteroids
        .iter()
        .map(|&coords| {
            let mut rays = BTreeMap::<Ray, VecDeque<(i32, Coords)>>::new();
            for &c2 in &asteroids {
                if c2 == coords {
                    continue;
                }
                let ydiff = i32::try_from(coords.y).unwrap() - i32::try_from(c2.y).unwrap();
                let xdiff = i32::try_from(c2.x).unwrap() - i32::try_from(coords.x).unwrap();
                let (ray, dist) = simplify_dist(ydiff, xdiff);
                rays.entry(ray).or_default().push_back((dist, c2))
            }
            rays
        })
        .max_by_key(|rays| rays.len())
        .unwrap();
    for (_, raypoints) in rays.iter_mut() {
        raypoints.make_contiguous().sort_by_key(|&(dist, _)| dist);
    }
    let mut destroyed = 0;
    loop {
        for (_, raypoints) in rays.iter_mut() {
            if let Some((_, coords)) = raypoints.pop_front() {
                destroyed += 1;
                if destroyed == 200 {
                    return coords.x * 100 + coords.y;
                }
            }
        }
    }
}

fn simplify_dist(y: i32, x: i32) -> (Ray, i32) {
    if x == 0 && y == 0 {
        return (Ray::new(0, 0), 0);
    } else if x == 0 {
        return (Ray::new(y.signum(), 0), y.abs());
    } else if y == 0 {
        return (Ray::new(0, x.signum()), x.abs());
    }
    let mut a = y.abs();
    let mut b = x.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    (Ray::new(y / a, x / a), a)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Ray {
    y: i32,
    x: i32,
}

impl Ray {
    fn new(y: i32, x: i32) -> Ray {
        Ray { y, x }
    }

    fn angle(&self) -> f64 {
        let angle = f64::try_from(self.y)
            .unwrap()
            .atan2(f64::try_from(self.x).unwrap());
        (FRAC_PI_2 - angle).rem_euclid(TAU)
    }
}

impl PartialOrd for Ray {
    fn partial_cmp(&self, other: &Ray) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ray {
    fn cmp(&self, other: &Ray) -> Ordering {
        self.angle().total_cmp(&other.angle())
    }
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_rays() {
        let mut rays = vec![
            Ray::new(1, 6),
            Ray::new(2, 3),
            Ray::new(3, 1),
            Ray::new(3, 2),
            Ray::new(1, 7),
            Ray::new(2, 7),
            Ray::new(2, 1),
            Ray::new(1, 4),
            Ray::new(2, 0),
            Ray::new(-1, -6),
            Ray::new(-1, -4),
            Ray::new(1, -7),
            Ray::new(0, 4),
            Ray::new(3, -7),
            Ray::new(3, -1),
            Ray::new(1, -8),
            Ray::new(0, -6),
            Ray::new(2, -2),
            Ray::new(2, 4),
            Ray::new(-1, 2),
            Ray::new(-1, 8),
            Ray::new(-1, 7),
            Ray::new(2, -3),
            Ray::new(2, -8),
            Ray::new(3, -2),
            Ray::new(1, 1),
            Ray::new(1, -3),
            Ray::new(2, -7),
            Ray::new(1, 5),
            Ray::new(1, 3),
        ];
        rays.sort();
        assert_eq!(
            rays,
            [
                Ray::new(2, 0),
                Ray::new(3, 1),
                Ray::new(2, 1),
                Ray::new(3, 2),
                Ray::new(1, 1),
                Ray::new(2, 3),
                Ray::new(2, 4),
                Ray::new(1, 3),
                Ray::new(2, 7),
                Ray::new(1, 4),
                Ray::new(1, 5),
                Ray::new(1, 6),
                Ray::new(1, 7),
                Ray::new(0, 4),
                Ray::new(-1, 8),
                Ray::new(-1, 7),
                Ray::new(-1, 2),
                Ray::new(-1, -4),
                Ray::new(-1, -6),
                Ray::new(0, -6),
                Ray::new(1, -8),
                Ray::new(1, -7),
                Ray::new(2, -8),
                Ray::new(2, -7),
                Ray::new(1, -3),
                Ray::new(3, -7),
                Ray::new(2, -3),
                Ray::new(2, -2),
                Ray::new(3, -2),
                Ray::new(3, -1),
            ]
        );
    }

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            ".#..##.###...#######\n",
            "##.############..##.\n",
            ".#.######.########.#\n",
            ".###.#######.####.#.\n",
            "#####.##.#.##.###.##\n",
            "..#####..#.#########\n",
            "####################\n",
            "#.####....###.#.#.##\n",
            "##.#################\n",
            "#####.##.###..####..\n",
            "..######..##.#######\n",
            "####.##.####...##..#\n",
            ".#####..#.######.###\n",
            "##...#.##########...\n",
            "#.##########.#######\n",
            ".####.#.###.###.#.##\n",
            "....##.##.###..#####\n",
            ".#.#.###########.###\n",
            "#.#.#.#####.####.###\n",
            "###.##.####.##.#..##\n",
        ));
        assert_eq!(solve(input), 802);
    }
}
