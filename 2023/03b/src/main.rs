use adventutil::Input;
use adventutil::grid::{Coords, Grid};
use std::collections::{HashMap, HashSet};

fn solve(input: Input) -> u32 {
    let grid = input.parse::<Grid<char>>();
    let mut gears2numbers: HashMap<Coords, Vec<u32>> = HashMap::new();
    for y in 0..grid.height() {
        let mut number = None;
        let mut gears: HashSet<Coords> = HashSet::new();
        for x in 0..grid.width() {
            let ch = grid[(y, x)];
            if let Some(digit) = ch.to_digit(10) {
                number = Some(number.unwrap_or_default() * 10 + digit);
                let cell = grid.get_cell((y, x)).unwrap();
                for adj in cell.adjacent() {
                    if *adj.get() == '*' {
                        gears.insert(adj.coords());
                    }
                }
            } else if let Some(n) = number.take() {
                for &g in &gears {
                    gears2numbers.entry(g).or_default().push(n);
                }
                gears.clear();
            }
        }
        if let Some(n) = number.take() {
            for g in gears {
                gears2numbers.entry(g).or_default().push(n);
            }
        }
    }
    gears2numbers
        .into_values()
        .filter_map(|nums| {
            if let [a, b] = nums.as_slice() {
                Some(a * b)
            } else {
                None
            }
        })
        .sum()
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
        assert_eq!(solve(input), 467835);
    }
}
