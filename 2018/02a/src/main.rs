use adventutil::counter::Counter;
use adventutil::Input;

fn solve(input: Input) -> usize {
    let (twos, threes): (Vec<_>, Vec<_>) = input.lines().map(box_properties).unzip();
    twos.into_iter().filter(|&b| b).count() * threes.into_iter().filter(|&b| b).count()
}

fn box_properties(boxid: String) -> (bool, bool) {
    let counts = boxid.chars().collect::<Counter<char>>();
    let mut has_two = false;
    let mut has_three = false;
    for qty in counts.into_values() {
        if qty == 2 {
            has_two = true;
        }
        if qty == 3 {
            has_three = true;
        }
    }
    (has_two, has_three)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", false, false)]
    #[case("bababc", true, true)]
    #[case("abbcde", true, false)]
    #[case("abcccd", false, true)]
    #[case("aabcdd", true, false)]
    #[case("abcdee", true, false)]
    #[case("ababab", false, true)]
    fn test_box_properties(#[case] boxid: &str, #[case] two: bool, #[case] three: bool) {
        assert_eq!(box_properties(boxid.into()), (two, three));
    }

    #[test]
    fn test_checksum() {
        let input = Input::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n");
        assert_eq!(solve(input), 12);
    }
}
