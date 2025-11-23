use adventutil::Input;
use adventutil::counter::Counter;

fn solve(input: Input) -> String {
    let input = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut output = String::new();
    for i in 0..(input[0].len()) {
        let counts = input.iter().map(|ln| ln[i]).collect::<Counter<_>>();
        let (c, _) = counts.into_iter().min_by_key(|&(_, qty)| qty).unwrap();
        output.push(c);
    }
    output
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
            "eedadn\n", "drvtee\n", "eandsr\n", "raavrd\n", "atevrs\n", "tsrnev\n", "sdttsa\n",
            "rasrtv\n", "nssdts\n", "ntnada\n", "svetve\n", "tesnvt\n", "vntsnd\n", "vrdear\n",
            "dvrsen\n", "enarar\n",
        ));
        assert_eq!(solve(input), "advent");
    }
}
