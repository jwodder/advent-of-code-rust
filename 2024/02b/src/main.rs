use adventutil::Input;

#[allow(clippy::if_same_then_else)]
#[allow(clippy::needless_bool)]
fn is_tolerated_gradual<I: IntoIterator<Item = u32>>(report: I) -> bool {
    let report = report.into_iter().collect::<Vec<_>>();
    is_gradual(report.iter().copied())
        || (0..(report.len())).any(|i| {
            let mut dampened = report.clone();
            dampened.remove(i);
            is_gradual(dampened)
        })
}

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
            is_tolerated_gradual(
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
    #[case([1, 3, 2, 4, 5], true)]
    #[case([8, 6, 4, 4, 1], true)]
    #[case([1, 3, 6, 7, 9], true)]
    fn test_is_tolerated_gradual(#[case] report: [u32; 5], #[case] r: bool) {
        assert_eq!(is_tolerated_gradual(report), r);
    }
}
