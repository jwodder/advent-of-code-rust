use adventutil::counter::Counter;
use adventutil::Input;

fn checksum<I, S>(iter: I) -> usize
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let (twos, threes): (Vec<_>, Vec<_>) = iter.into_iter().map(box_properties).unzip();
    twos.into_iter().filter(|&b| b).count() * threes.into_iter().filter(|&b| b).count()
}

fn box_properties<S: AsRef<str>>(boxid: S) -> (bool, bool) {
    let counts = boxid.as_ref().chars().collect::<Counter<char>>();
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
    println!("{}", checksum(Input::from_env().lines()));
}

#[cfg(test)]
mod test {
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
        assert_eq!(box_properties(boxid), (two, three));
    }

    #[test]
    fn test_checksum() {
        let boxids = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(checksum(boxids), 12);
    }
}
