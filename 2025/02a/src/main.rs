use adventutil::Input;

fn invalid_sum(start: u64, end: u64) -> u64 {
    let mut accum = 0;
    for i in start..=end {
        let s = format!("{i}");
        if s.len().is_multiple_of(2) {
            let j = s.len() / 2;
            if s[..j] == s[j..] {
                accum += i;
            }
        }
    }
    accum
}

fn solve(input: Input) -> u64 {
    input
        .read()
        .trim()
        .split(',')
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            invalid_sum(start, end)
        })
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
        let input = Input::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(solve(input), 1227775554);
    }
}
