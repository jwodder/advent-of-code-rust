use adventutil::grid::Grid;
use adventutil::Input;

fn solve(input: Input) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for para in input.paragraphs() {
        let diag = Grid::from_drawing(&para).unwrap();
        let heights = (0..(diag.width()))
            .map(|x| (0..(diag.height())).filter(|&y| diag[(y, x)]).count() - 1)
            .collect::<Vec<_>>();
        if (0..(diag.width())).all(|x| diag[(0, x)]) {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }
    let mut matches = 0;
    for lck in locks {
        for k in &keys {
            if std::iter::zip(&lck, k).all(|(&a, &b)| a + b < 6) {
                matches += 1;
            }
        }
    }
    matches
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
            "#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....\n\n",
            "#####\n##.##\n.#.##\n...##\n...#.\n...#.\n.....\n\n",
            ".....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####\n\n",
            ".....\n.....\n#.#..\n###..\n###.#\n###.#\n#####\n\n",
            ".....\n.....\n.....\n#....\n#.#..\n#.#.#\n#####\n",
        ));
        assert_eq!(solve(input), 3);
    }
}
