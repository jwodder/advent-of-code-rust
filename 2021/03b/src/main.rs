use adventutil::grid::Grid;
use adventutil::{FromBits, Input};

fn solve(input: Input) -> u32 {
    let gr = input.parse::<Grid<u8>>().map(|i| i == 1);
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
        let (ones, zeroes) = col.into_iter().fold((0, 0), |(ones, zeroes), &b| {
            if b {
                (ones + 1, zeroes)
            } else {
                (ones, zeroes + 1)
            }
        });
        let target = selector(ones, zeroes);
        gr = gr.filter_rows(|row| row[x] == target).unwrap();
        if gr.height() == 1 {
            return u32::from_bits(gr.into_rows().next().unwrap());
        }
    }
    panic!("Grid filtering did not converge on a single row");
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = Input::from(concat!(
            "00100\n", "11110\n", "10110\n", "10111\n", "10101\n", "01111\n", "00111\n", "11100\n",
            "10000\n", "11001\n", "00010\n", "01010\n",
        ));
        assert_eq!(solve(input), 230);
    }
}
