use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::iter::{once, Sum};
use std::ops::Add;
use std::str::FromStr;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    #[allow(unused)]
    calories: i32,
}

impl Ingredient {
    fn score(&self, qty: i32) -> Score {
        Score {
            capacity: self.capacity * qty,
            durability: self.durability * qty,
            flavor: self.flavor * qty,
            texture: self.texture * qty,
        }
    }
}

impl FromStr for Ingredient {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Ingredient, ParseError> {
        let mut parser = PullParser::new(s);
        let _name = parser.parse_to::<String, _>(':')?;
        parser.skip(" capacity ")?;
        let capacity = parser.parse_to::<i32, _>(',')?;
        parser.skip(" durability ")?;
        let durability = parser.parse_to::<i32, _>(',')?;
        parser.skip(" flavor ")?;
        let flavor = parser.parse_to::<i32, _>(',')?;
        parser.skip(" texture ")?;
        let texture = parser.parse_to::<i32, _>(',')?;
        parser.skip(" calories ")?;
        let calories = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

#[derive(Default)]
struct Score {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
}

impl Score {
    fn product(self) -> i32 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, rhs: Score) -> Score {
        Score {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
        }
    }
}

impl Sum for Score {
    fn sum<I: Iterator<Item = Score>>(iter: I) -> Score {
        iter.fold(Score::default(), |a, b| a + b)
    }
}

fn solve(input: Input) -> i32 {
    let ingredients = input.parse_lines::<Ingredient>().collect::<Vec<_>>();
    partitions(100, ingredients.len())
        .map(|p| {
            ingredients
                .iter()
                .zip(p)
                .map(|(i, qty)| i.score(qty))
                .sum::<Score>()
                .product()
        })
        .max()
        .unwrap()
}

// Partitions of `qty` unlabelled elements into `bins` labelled bins
fn partitions(qty: i32, bins: usize) -> Box<dyn Iterator<Item = Vec<i32>>> {
    if bins == 0 {
        panic!("Partitioning into 0 bins");
    } else if bins == 1 {
        Box::new(once(vec![qty]))
    } else {
        Box::new((0..=qty).flat_map(move |i| {
            partitions(qty - i, bins - 1).map(move |mut p| {
                p.push(i);
                p
            })
        }))
    }
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
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3\n",
        ));
        assert_eq!(solve(input), 62842880);
    }
}
