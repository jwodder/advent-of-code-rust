use adventutil::Input;
use adventutil::ranges::parse_range;

fn invalid_sum(r: std::ops::RangeInclusive<u64>) -> u64 {
    let mut accum = 0;
    'i: for i in r {
        let s = format!("{i}");
        for m in 1..=(s.len() / 2) {
            if s.len().is_multiple_of(m) {
                let j = s.len() / m;
                if (1..j).all(|n| s[..m] == s[(m * n)..(m * (n + 1))]) {
                    accum += i;
                    continue 'i;
                }
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
        .map(|s| invalid_sum(parse_range::<u64>(s).unwrap()))
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
        assert_eq!(solve(input), 4174379265);
    }
}
