use adventutil::Input;

fn is_gradual<I: IntoIterator<Item = u32>>(report: I) -> bool {
    let mut iter = report.into_iter();
    let Some(prev) = iter.next() else {
        return true;
    };
    let Some(n) = iter.next() else {
        return true;
    };
    let ascending = n > prev;
    if !(1..=3).contains(&n.abs_diff(prev)) {
        return false;
    }
    let mut prev = n;
    for n in iter {
        if (ascending && n <= prev) || (!ascending && n >= prev) {
            return false;
        }
        if !(1..=3).contains(&n.abs_diff(prev)) {
            return false;
        }
        prev = n;
    }
    true
}

fn solve(input: Input) -> usize {
    input
        .lines()
        .filter(|ln| {
            is_gradual(
                ln.split_ascii_whitespace()
                    .map(|s| s.parse::<u32>().unwrap()),
            )
        })
        .count()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([7, 6, 4, 2, 1], true)]
    #[case([1, 2, 7, 8, 9], false)]
    #[case([9, 7, 6, 2, 1], false)]
    #[case([1, 3, 2, 4, 5], false)]
    #[case([8, 6, 4, 4, 1], false)]
    #[case([1, 3, 6, 7, 9], true)]
    fn examples(#[case] report: [u32; 5], #[case] r: bool) {
        assert_eq!(is_gradual(report), r);
    }
}
