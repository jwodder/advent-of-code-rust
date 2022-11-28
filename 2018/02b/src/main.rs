use adventutil::Input;

fn solve<I, S>(iter: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let strs = iter.into_iter().collect::<Vec<_>>();
    for i in 0..strs.len() {
        for j in (i + 1)..strs.len() {
            let s1 = strs[i].as_ref();
            let s2 = strs[j].as_ref();
            if let Some(k) = diff_index(s1, s2) {
                return s1
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| (i != k).then_some(c))
                    .collect();
            }
        }
    }
    panic!("No matching box IDs found");
}

fn diff_index(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut iter = s1
        .chars()
        .zip(s2.chars())
        .enumerate()
        .filter_map(|(i, (c1, c2))| (c1 != c2).then_some(i));
    let k = iter.next()?;
    iter.next().is_none().then_some(k)
}

fn main() {
    println!("{}", solve(Input::from_env().lines()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let boxids = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(solve(boxids), "fgij");
    }
}
