use adventutil::Input;
use itertools::Itertools;

fn ways_to_win(time: u32, dist: u32) -> usize {
    (1..time).filter(|&t| (time - t) * t > dist).count()
}

fn solve(input: Input) -> usize {
    let (times, distances) = input
        .lines()
        .collect_tuple()
        .expect("Input is not exactly two lines");
    let times = times
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|word| word.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let distances = distances
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|word| word.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    std::iter::zip(times, distances)
        .map(|(t, d)| ways_to_win(t, d))
        .product()
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
        assert_eq!(solve(input), 288);
    }
}
