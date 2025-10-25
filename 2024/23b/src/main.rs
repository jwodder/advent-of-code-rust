use adventutil::Input;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

fn solve(input: Input) -> String {
    let mut connected = HashMap::<String, BTreeSet<String>>::new();
    for ln in input.lines() {
        let (a, b) = ln.split_once('-').unwrap();
        let a = a.to_owned();
        let b = b.to_owned();
        connected.entry(a.clone()).or_default().insert(b.clone());
        connected.entry(b).or_default().insert(a);
    }
    let mut complete = connected
        .keys()
        .map(|node| BTreeSet::from([node.clone()]))
        .collect::<BTreeSet<_>>();
    loop {
        let mut complete2 = BTreeSet::new();
        for (node, xs) in &connected {
            for graph in &complete {
                if xs.is_superset(graph) {
                    let mut graph2 = graph.clone();
                    graph2.insert(node.clone());
                    complete2.insert(graph2);
                }
            }
        }
        if complete2.is_empty() {
            assert_eq!(
                complete.len(),
                1,
                "Terminated with {} maximal complete graphs",
                complete.len()
            );
            let mut graph = Vec::from_iter(complete.into_iter().next().unwrap());
            graph.sort_unstable();
            return graph.into_iter().join(",");
        }
        complete = complete2;
    }
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
        assert_eq!(solve(input), "co,de,ka,ta");
    }
}
