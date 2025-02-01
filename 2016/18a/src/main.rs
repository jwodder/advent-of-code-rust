use adventutil::Input;

fn advance(traps: &[bool]) -> Vec<bool> {
    (0..(traps.len()))
        .map(|i| {
            let left = i.checked_sub(1).is_some_and(|i| traps[i]);
            let center = traps[i];
            let right = traps.get(i + 1).copied().unwrap_or_default();
            matches!(
                (left, center, right),
                (true, true | false, false) | (false, true | false, true)
            )
        })
        .collect()
}

fn safe_in_rows(start: &str, rows: usize) -> usize {
    let traps = start.chars().map(|ch| ch == '^').collect::<Vec<_>>();
    std::iter::successors(Some(traps), |t| Some(advance(t)))
        .take(rows)
        .map(|r| r.into_iter().filter(|&b| !b).count())
        .sum()
}

fn solve(input: Input) -> usize {
    safe_in_rows(input.read().trim(), 40)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(safe_in_rows(".^^.^.^^^^", 10), 38);
    }
}
