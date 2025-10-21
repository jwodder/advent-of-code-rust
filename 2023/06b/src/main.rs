use adventutil::Input;
use itertools::Itertools;

fn ways_to_win(time: u64, dist: u64) -> usize {
    (1..time).filter(|&t| (time - t) * t > dist).count()
}

fn solve(input: Input) -> usize {
    let (times, distances) = input
        .lines()
        .collect_tuple()
        .expect("Input is not exactly two lines");
    let time = times
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distances
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    ways_to_win(time, distance)
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
            "Time:      7  15   30\n",
            "Distance:  9  40  200\n",
        ));
        assert_eq!(solve(input), 71503);
    }
}
