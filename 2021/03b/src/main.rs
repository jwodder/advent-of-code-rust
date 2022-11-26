use adventutil::grid::Grid;
use adventutil::Input;

fn parse_bits(s: &str) -> Grid<bool> {
    s.parse::<Grid<u8>>()
        .expect("Error parsing input")
        .map(|i| i == 1)
}

fn solve(gr: Grid<bool>) -> u32 {
    let o2gen = filter_rows(gr.clone(), |ones, zeroes| ones >= zeroes);
    let co2scrub = filter_rows(gr, |ones, zeroes| ones < zeroes);
    o2gen * co2scrub
}

// `selector` will be passed the number of true bits and the number of false
// bits in a column, and it must return the verity of the bits to select.
fn filter_rows<F>(mut gr: Grid<bool>, mut selector: F) -> u32
where
    F: FnMut(usize, usize) -> bool,
{
    for x in 0..gr.width() {
        let col = gr.get_column(x).unwrap();
        let (ones, zeroes): (Vec<bool>, Vec<bool>) = col.into_iter().partition(|b| **b);
        let target = selector(ones.len(), zeroes.len());
        gr = gr.filter_rows(|row| row[x] == target).unwrap();
        if gr.height() == 1 {
            return bits2num(gr.into_rows().next().unwrap());
        }
    }
    panic!("Grid filtering did not converge on a single row");
}

fn bits2num<I: IntoIterator<Item = bool>>(bits: I) -> u32 {
    bits.into_iter().fold(0, |n, b| (n << 1) + u32::from(b))
}

fn main() {
    let report = parse_bits(&Input::from_env().read());
    println!("{}", solve(report));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let report = parse_bits(concat!(
            "00100\n", "11110\n", "10110\n", "10111\n", "10101\n", "01111\n", "00111\n", "11100\n",
            "10000\n", "11001\n", "00010\n", "01010\n",
        ));
        assert_eq!(solve(report), 230);
    }
}
