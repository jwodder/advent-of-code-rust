use adventutil::grid::Grid;
use adventutil::Input;

fn solve(input: Input) -> u32 {
    let gr = input.parse::<Grid<u8>>().map(|i| i == 1);
    let gamma = gr
        .columns()
        .map(|col| {
            let (ones, zeroes): (Vec<bool>, Vec<bool>) = col.into_iter().partition(|b| **b);
            ones.len() > zeroes.len()
        })
        .collect::<Vec<_>>();
    let epsilon = gamma.iter().map(|&b| !b).collect::<Vec<_>>();
    bits2num(gamma) * bits2num(epsilon)
}

fn bits2num<I: IntoIterator<Item = bool>>(bits: I) -> u32 {
    bits.into_iter().fold(0, |n, b| (n << 1) + u32::from(b))
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "00100\n", "11110\n", "10110\n", "10111\n", "10101\n", "01111\n", "00111\n", "11100\n",
            "10000\n", "11001\n", "00010\n", "01010\n",
        ));
        assert_eq!(solve(input), 198);
    }
}
