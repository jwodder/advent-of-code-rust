use adventutil::Input;
use std::collections::{HashMap, HashSet};

fn solve(input: Input) -> usize {
    let mut connected = HashMap::<String, HashSet<String>>::new();
    for ln in input.lines() {
        let (a, b) = ln.split_once('-').unwrap();
        let a = a.to_owned();
        let b = b.to_owned();
        connected.entry(a.clone()).or_default().insert(b.clone());
        connected.entry(b).or_default().insert(a);
    }
    let mut qty = 0;
    for (node1, xs) in &connected {
        for node2 in xs {
            for node3 in &connected[node2] & xs {
                if node1.starts_with('t') || node2.starts_with('t') || node3.starts_with('t') {
                    qty += 1;
                }
            }
        }
    }
    qty / 6
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from(concat!(
            "kh-tc\n", "qp-kh\n", "de-cg\n", "ka-co\n", "yn-aq\n", "qp-ub\n", "cg-tb\n", "vc-aq\n",
            "tb-ka\n", "wh-tc\n", "yn-cg\n", "kh-ub\n", "ta-co\n", "de-co\n", "tc-td\n", "tb-wq\n",
            "wh-td\n", "ta-ka\n", "td-qp\n", "aq-cg\n", "wq-ub\n", "ub-vc\n", "de-ta\n", "wq-aq\n",
            "wq-vc\n", "wh-yn\n", "ka-de\n", "kh-ta\n", "co-tc\n", "wh-qp\n", "tb-vc\n", "td-yn\n",
        ));
        assert_eq!(solve(input), 7);
    }
}
