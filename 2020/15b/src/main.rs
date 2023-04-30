use adventutil::Input;
use std::collections::HashMap;

fn solve(input: Input) -> u32 {
    let starting = input.parse_csv_line::<u32>();
    let mut last_seen = HashMap::new();
    let mut i = 1;
    let mut prev = 0;
    for number in starting {
        last_seen.insert(number, i);
        prev = number;
        i += 1;
    }
    last_seen.remove(&prev);
    while i <= 30_000_000 {
        let number = match last_seen.get(&prev) {
            Some(&step) => (i - 1) - step,
            None => 0,
        };
        last_seen.insert(prev, i - 1);
        prev = number;
        i += 1;
    }
    prev
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("0,3,6", 175594)]
    #[case("1,3,2", 2578)]
    #[case("2,1,3", 3544142)]
    #[case("1,2,3", 261214)]
    #[case("2,3,1", 6895259)]
    #[case("3,2,1", 18)]
    #[case("3,1,2", 362)]
    fn test_solve(#[case] s: &'static str, #[case] output: u32) {
        assert_eq!(solve(Input::from(s)), output);
    }
}
