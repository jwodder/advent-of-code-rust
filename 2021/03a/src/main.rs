use adventutil::grid::Grid;
use adventutil::{FromBits, Input};

fn solve(input: Input) -> u32 {
    let gr = input.parse::<Grid<u8>>().map(|i| i == 1);
    let gamma = gr
        .columns()
        .map(|col| {
            let (ones, zeroes) = col.into_iter().fold((0, 0), |(ones, zeroes), &b| {
                if b {
                    (ones + 1, zeroes)
                } else {
                    (ones, zeroes + 1)
                }
            });
            ones > zeroes
        })
        .collect::<Vec<_>>();
    let epsilon = gamma.iter().map(|&b| !b).collect::<Vec<_>>();
    u32::from_bits(gamma) * u32::from_bits(epsilon)
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
            "00100\n", "11110\n", "10110\n", "10111\n", "10101\n", "01111\n", "00111\n", "11100\n",
            "10000\n", "11001\n", "00010\n", "01010\n",
        ));
        assert_eq!(solve(input), 198);
    }
}
