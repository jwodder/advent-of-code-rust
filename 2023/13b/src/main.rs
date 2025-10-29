use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> usize {
    let mut summary = 0;
    for para in input.paragraphs() {
        let pattern = Grid::from_drawing(&para).unwrap();
        if let Some(i) = (1..pattern.width()).find(|&i| {
            let width = i.min(pattern.width() - i);
            std::iter::zip(((i - width)..i).rev(), i..(i + width))
                .flat_map(|(x1, x2)| {
                    let grid = &pattern;
                    (0..pattern.height()).filter(move |&y| grid[(y, x1)] != grid[(y, x2)])
                })
                .count()
                == 1
        }) {
            summary += i;
        } else if let Some(i) = (1..pattern.height()).find(|&i| {
            let height = i.min(pattern.height() - i);
            std::iter::zip(((i - height)..i).rev(), i..(i + height))
                .flat_map(|(y1, y2)| {
                    let grid = &pattern;
                    (0..pattern.width()).filter(move |&x| grid[(y1, x)] != grid[(y2, x)])
                })
                .count()
                == 1
        }) {
            summary += i * 100;
        } else {
            panic!("No symmetry for pattern");
        }
    }
    summary
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
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        ));
        assert_eq!(solve(input), 400);
    }
}
