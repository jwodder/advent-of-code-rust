use adventutil::counter::Counter;
use adventutil::gridgeom::Point;
use adventutil::maxtracker::MaxTracker;
use adventutil::{Input, unordered_pairs};

fn solve(input: Input) -> i32 {
    let mut y_qtys = Counter::new();
    let mut x_qtys = Counter::new();
    let mut max_y_tracker = MaxTracker::new();
    let mut max_x_tracker = MaxTracker::new();
    let mut galaxies = Vec::new();
    for (y, ln) in input.lines().enumerate() {
        for (x, c) in ln.chars().enumerate() {
            if c == '#' {
                y_qtys.add(y);
                x_qtys.add(x);
                max_y_tracker.add(y);
                max_x_tracker.add(x);
                galaxies.push((y, x));
            }
        }
    }
    let max_y = max_y_tracker.get().unwrap_or_default();
    let max_x = max_x_tracker.get().unwrap_or_default();
    let mut y_increases = vec![0; max_y + 1];
    for y in 0..=max_y {
        if y_qtys[&y] == 0 {
            for v in y_increases.iter_mut().skip(y) {
                *v += 1;
            }
        }
    }
    let mut x_increases = vec![0; max_x + 1];
    for x in 0..=max_x {
        if x_qtys[&x] == 0 {
            for v in x_increases.iter_mut().skip(x) {
                *v += 1;
            }
        }
    }
    let galaxies = galaxies
        .into_iter()
        .map(|(y, x)| Point {
            y: i32::try_from(y + y_increases[y]).unwrap(),
            x: i32::try_from(x + x_increases[x]).unwrap(),
        })
        .collect::<Vec<_>>();
    unordered_pairs(&galaxies)
        .map(|(&g1, &g2)| (g1 - g2).taxicab_len())
        .sum()
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
            "...#......\n",
            ".......#..\n",
            "#.........\n",
            "..........\n",
            "......#...\n",
            ".#........\n",
            ".........#\n",
            "..........\n",
            ".......#..\n",
            "#...#.....\n",
        ));
        assert_eq!(solve(input), 374);
    }
}
