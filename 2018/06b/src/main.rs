use adventutil::Input;
use adventutil::gridgeom::{Point, PointBounds};

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split_once(',').unwrap();
    let x = x.trim().parse::<i32>().unwrap();
    let y = y.trim().parse::<i32>().unwrap();
    Point { x, y }
}

fn solve(input: Input, max_sum: i32) -> usize {
    let coords = input.lines().map(|s| parse_point(&s)).collect::<Vec<_>>();
    let bounds = PointBounds::for_points(coords.iter().copied()).unwrap();
    let mut qty = 0;
    for y in bounds.min_y..=bounds.max_y {
        for x in bounds.min_x..=bounds.max_x {
            let p = Point { x, y };
            let dist_sum: i32 = coords.iter().map(|&c| (p - c).taxicab_len()).sum();
            if dist_sum < max_sum {
                qty += 1;
            }
        }
    }
    qty
}

fn main() {
    println!("{}", solve(Input::from_env(), 10000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n");
        assert_eq!(solve(input, 32), 16);
    }
}
