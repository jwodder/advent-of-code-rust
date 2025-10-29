use adventutil::Input;
use adventutil::grid::Grid;

fn solve(input: Input) -> usize {
    let mut summary = 0;
    for para in input.paragraphs() {
        let pattern = Grid::from_drawing(&para).unwrap();
        if let Some(i) = (1..pattern.width()).find(|&i| {
            let width = i.min(pattern.width() - i);
            std::iter::zip(((i - width)..i).rev(), i..(i + width))
                .all(|(x1, x2)| (0..pattern.height()).all(|y| pattern[(y, x1)] == pattern[(y, x2)]))
        }) {
            summary += i;
        } else if let Some(i) = (1..pattern.height()).find(|&i| {
            let height = i.min(pattern.height() - i);
            std::iter::zip(((i - height)..i).rev(), i..(i + height))
                .all(|(y1, y2)| (0..pattern.width()).all(|x| pattern[(y1, x)] == pattern[(y2, x)]))
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
        assert_eq!(solve(input), 405);
    }
}
