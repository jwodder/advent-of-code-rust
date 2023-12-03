use adventutil::grid::Grid;
use adventutil::Input;

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<char>>();
    let mut total = 0;
    for y in 0..grid.height() {
        let mut number: Option<u32> = None;
        let mut adjacent = false;
        for x in 0..grid.width() {
            let ch = grid[(y, x)];
            if let Some(digit) = ch.to_digit(10) {
                number = Some(number.unwrap_or_default() * 10 + digit);
                if !adjacent {
                    let cell = grid.get_cell((y, x)).unwrap();
                    adjacent = cell
                        .adjacent()
                        .any(|c| *c.get() != '.' && !c.get().is_ascii_digit());
                }
            } else if let Some(n) = number.take() {
                if adjacent {
                    total += n;
                }
                adjacent = false;
            }
        }
        if let Some(n) = number.take() {
            if adjacent {
                total += n;
            }
        }
    }
    total
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
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598..\n",
        ));
        assert_eq!(solve(input), 4361);
    }
}
