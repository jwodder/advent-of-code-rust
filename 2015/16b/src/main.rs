use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
enum Property {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl Property {
    fn compare(&self, fvalue: usize, svalue: usize) -> bool {
        use Property::*;
        match self {
            Cats | Trees => fvalue < svalue,
            Pomeranians | Goldfish => fvalue > svalue,
            _ => fvalue == svalue,
        }
    }
}

impl std::str::FromStr for Property {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Property, ParseError> {
        use Property::*;
        match s {
            "children" => Ok(Children),
            "cats" => Ok(Cats),
            "samoyeds" => Ok(Samoyeds),
            "pomeranians" => Ok(Pomeranians),
            "akitas" => Ok(Akitas),
            "vizslas" => Ok(Vizslas),
            "goldfish" => Ok(Goldfish),
            "trees" => Ok(Trees),
            "cars" => Ok(Cars),
            "perfumes" => Ok(Perfumes),
            s => Err(ParseError::InvalidToken(s.into())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AuntSue {
    id: usize,
    properties: HashMap<Property, usize>,
}

impl std::str::FromStr for AuntSue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<AuntSue, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Sue ")?;
        let id = parser.parse_to::<usize, _>(':')?;
        let mut properties = HashMap::new();
        for i in 0..3 {
            parser.skip(Token::Whitespace)?;
            let prop = parser.parse_to::<Property, _>(':')?;
            parser.skip(Token::Whitespace)?;
            let value =
                parser.parse_to::<usize, _>(if i < 2 { Token::Char(',') } else { Token::Eof })?;
            properties.insert(prop, value);
        }
        Ok(AuntSue { id, properties })
    }
}

fn props_match(forensics: &HashMap<Property, usize>, sue: &AuntSue) -> bool {
    sue.properties
        .iter()
        .all(|(k, v)| k.compare(*forensics.get(k).unwrap(), *v))
}

fn solve(input: Input) -> usize {
    use Property::*;
    let forensics = HashMap::from([
        (Children, 3),
        (Cats, 7),
        (Samoyeds, 2),
        (Pomeranians, 3),
        (Akitas, 0),
        (Vizslas, 0),
        (Goldfish, 5),
        (Trees, 3),
        (Cars, 2),
        (Perfumes, 1),
    ]);
    input
        .parse_lines::<AuntSue>()
        .find(|sue| props_match(&forensics, sue))
        .unwrap()
        .id
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
