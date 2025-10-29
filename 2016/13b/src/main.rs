use adventutil::Input;
use std::collections::HashSet;

fn is_open(key: i32, x: i32, y: i32) -> bool {
    let value = x * x + 3 * x + 2 * x * y + y + y * y + key;
    value.count_ones().is_multiple_of(2)
}

fn solve(input: Input) -> usize {
    let key = input.parse::<i32>();
    let seed = (1, 1);
    let mut seen = HashSet::from([seed]);
    let mut queue = Vec::from([seed]);
    for _ in 0..50 {
        let mut newqueue = Vec::new();
        for (x, y) in queue {
            for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let x2 = x + i;
                let y2 = y + j;
                if x2 >= 0 && y2 >= 0 && is_open(key, x2, y2) && seen.insert((x2, y2)) {
                    newqueue.push((x2, y2));
                }
            }
        }
        queue = newqueue;
    }
    seen.len()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
