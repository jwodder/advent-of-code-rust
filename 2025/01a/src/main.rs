use adventutil::Input;

fn rotate(start: u8, motion: &str) -> u8 {
    if let Some(offset) = motion.strip_prefix("L").and_then(|s| s.parse::<u32>().ok()) {
        let offset = u8::try_from(offset % 100).unwrap();
        (start + 100 - offset) % 100
    } else if let Some(offset) = motion.strip_prefix("R").and_then(|s| s.parse::<u32>().ok()) {
        let offset = u8::try_from(offset % 100).unwrap();
        (start + offset) % 100
    } else {
        panic!("Invalid motion {motion:?}");
    }
}

fn solve(input: Input) -> usize {
    let mut pos = 50;
    let mut qty = 0;
    for ln in input.lines() {
        pos = rotate(pos, &ln);
        if pos == 0 {
            qty += 1;
        }
    }
    qty
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(11, "R8", 19)]
    #[case(19, "L19", 0)]
    #[case(0, "L1", 99)]
    #[case(99, "R1", 0)]
    #[case(5, "L10", 95)]
    #[case(95, "R5", 0)]
    fn rotate(#[case] start: u8, #[case] motion: &str, #[case] result: u8) {
        assert_eq!(super::rotate(start, motion), result);
    }

    #[test]
    fn example1() {
        let input = Input::from("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n");
        assert_eq!(solve(input), 3);
    }
}
