use adventutil::Input;
use itertools::Itertools;

fn solve(input: Input) -> u32 {
    let mut joltages = input.parse_lines::<u32>().collect::<Vec<_>>();
    joltages.push(0);
    joltages.sort_unstable();
    let mut delta1 = 0;
    let mut delta3 = 1;
    for (j1, j2) in joltages.into_iter().tuple_windows() {
        match j2 - j1 {
            1 => delta1 += 1,
            2 => (),
            3 => delta3 += 1,
            d => panic!("Overly large joltage difference: {d}"),
        }
    }
    delta1 * delta3
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = Input::from("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n");
        assert_eq!(solve(input), 35);
    }

    #[test]
    fn example2() {
        let input = Input::from(concat!(
            "28\n", "33\n", "18\n", "42\n", "31\n", "14\n", "46\n", "20\n", "48\n", "47\n", "24\n",
            "23\n", "49\n", "45\n", "19\n", "38\n", "39\n", "11\n", "1\n", "32\n", "25\n", "35\n",
            "8\n", "17\n", "7\n", "9\n", "4\n", "2\n", "34\n", "10\n", "3\n",
        ));
        assert_eq!(solve(input), 220);
    }
}
