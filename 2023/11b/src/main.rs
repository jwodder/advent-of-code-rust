use adventutil::counter::Counter;
use adventutil::maxtracker::MaxTracker;
use adventutil::{Input, unordered_pairs};

fn solve(input: Input) -> usize {
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
                *v += 999_999;
            }
        }
    }
    let mut x_increases = vec![0; max_x + 1];
    for x in 0..=max_x {
        if x_qtys[&x] == 0 {
            for v in x_increases.iter_mut().skip(x) {
                *v += 999_999;
            }
        }
    }
    let galaxies = galaxies
        .into_iter()
        .map(|(y, x)| (y + y_increases[y], x + x_increases[x]))
        .collect::<Vec<_>>();
    unordered_pairs(&galaxies)
        .map(|(&(y1, x1), &(y2, x2))| y1.abs_diff(y2) + x1.abs_diff(x2))
        .sum()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
