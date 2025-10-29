use adventutil::{Input, dijkstra_length};

fn is_open(key: i32, x: i32, y: i32) -> bool {
    let value = x * x + 3 * x + 2 * x * y + y + y * y + key;
    value.count_ones().is_multiple_of(2)
}

fn steps(key: i32, target_x: i32, target_y: i32) -> u32 {
    dijkstra_length(
        (1, 1),
        |&n| n == (target_x, target_y),
        |&(x, y)| {
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .map(move |(i, j)| (x + i, y + j))
                .filter(|&(i, j)| is_open(key, i, j))
                .map(|p| (p, 1))
        },
    )
    .unwrap()
}

fn solve(input: Input) -> u32 {
    let key = input.parse::<i32>();
    steps(key, 31, 39)
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(steps(10, 7, 4), 11);
    }
}
