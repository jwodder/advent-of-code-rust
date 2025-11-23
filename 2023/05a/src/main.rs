use adventutil::Input;
use adventutil::pullparser::{ParseError, PullParser, Token};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn seed2location(&self, seed: u32) -> u32 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        self.humidity_to_location.map(humidity)
    }
}

impl std::str::FromStr for Almanac {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Almanac, ParseError> {
        fn parse_map(parser: &mut PullParser<'_>) -> Result<Vec<MapEntry>, ParseError> {
            let mut entries = Vec::new();
            while parser.eof().is_err() {
                let line = parser.scan_to(Token::Newline)?;
                if line.is_empty() {
                    break;
                }
                entries.push(line.parse::<MapEntry>()?);
            }
            Ok(entries)
        }

        let mut almanac = Almanac::default();
        let mut parser = PullParser::new(s);
        parser.skip("seeds:")?;
        almanac.seeds = parser
            .scan_to(Token::Newline)?
            .split_whitespace()
            .map(str::parse::<u32>)
            .collect::<Result<Vec<_>, _>>()?;
        parser.skip(Token::Newline)?;

        parser.skip("seed-to-soil map:")?;
        parser.skip(Token::Newline)?;
        almanac.seed_to_soil.extend(parse_map(&mut parser)?);

        parser.skip("soil-to-fertilizer map:")?;
        parser.skip(Token::Newline)?;
        almanac.soil_to_fertilizer.extend(parse_map(&mut parser)?);

        parser.skip("fertilizer-to-water map:")?;
        parser.skip(Token::Newline)?;
        almanac.fertilizer_to_water.extend(parse_map(&mut parser)?);

        parser.skip("water-to-light map:")?;
        parser.skip(Token::Newline)?;
        almanac.water_to_light.extend(parse_map(&mut parser)?);

        parser.skip("light-to-temperature map:")?;
        parser.skip(Token::Newline)?;
        almanac.light_to_temperature.extend(parse_map(&mut parser)?);

        parser.skip("temperature-to-humidity map:")?;
        parser.skip(Token::Newline)?;
        almanac
            .temperature_to_humidity
            .extend(parse_map(&mut parser)?);

        parser.skip("humidity-to-location map:")?;
        parser.skip(Token::Newline)?;
        almanac.humidity_to_location.extend(parse_map(&mut parser)?);

        Ok(almanac)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    fn map(&self, src: u32) -> u32 {
        match self
            .entries
            .binary_search_by_key(&src, |entry| entry.src_start)
        {
            Ok(i) => self.entries[i].map(src),
            Err(i) => match i.checked_sub(1) {
                Some(i) => self.entries[i].map(src),
                None => return src,
            },
        }
        .unwrap_or(src)
    }
}

impl Extend<MapEntry> for Map {
    fn extend<I: IntoIterator<Item = MapEntry>>(&mut self, iter: I) {
        self.entries.extend(iter);
        self.entries.sort_unstable_by_key(|entry| entry.src_start);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct MapEntry {
    dest_start: u32,
    src_start: u32,
    length: u32,
}

impl MapEntry {
    fn map(&self, src: u32) -> Option<u32> {
        let diff = src - self.src_start;
        (diff < self.length).then_some(self.dest_start + diff)
    }
}

impl std::str::FromStr for MapEntry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<MapEntry, ParseError> {
        let mut parser = PullParser::new(s);
        let dest_start = parser.parse_to::<u32, _>(Token::Whitespace)?;
        let src_start = parser.parse_to::<u32, _>(Token::Whitespace)?;
        let length = parser.parse_to::<u32, _>(Token::Eof)?;
        Ok(MapEntry {
            dest_start,
            src_start,
            length,
        })
    }
}

fn solve(input: Input) -> u32 {
    // Don't use Input::parse() here, as that trims the input before parsing.
    let almanac = input.read().parse::<Almanac>().unwrap();
    almanac
        .seeds
        .iter()
        .copied()
        .map(|seed| almanac.seed2location(seed))
        .min()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    static EXAMPLE: &str = concat!(
        "seeds: 79 14 55 13\n",
        "\n",
        "seed-to-soil map:\n",
        "50 98 2\n",
        "52 50 48\n",
        "\n",
        "soil-to-fertilizer map:\n",
        "0 15 37\n",
        "37 52 2\n",
        "39 0 15\n",
        "\n",
        "fertilizer-to-water map:\n",
        "49 53 8\n",
        "0 11 42\n",
        "42 0 7\n",
        "57 7 4\n",
        "\n",
        "water-to-light map:\n",
        "88 18 7\n",
        "18 25 70\n",
        "\n",
        "light-to-temperature map:\n",
        "45 77 23\n",
        "81 45 19\n",
        "68 64 13\n",
        "\n",
        "temperature-to-humidity map:\n",
        "0 69 1\n",
        "1 0 69\n",
        "\n",
        "humidity-to-location map:\n",
        "60 56 37\n",
        "56 93 4\n",
    );

    #[test]
    fn example1() {
        let input = Input::from(EXAMPLE);
        assert_eq!(solve(input), 35);
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn seed_to_location(#[case] seed: u32, #[case] loc: u32) {
        let almanac = EXAMPLE.parse::<Almanac>().unwrap();
        assert_eq!(almanac.seed2location(seed), loc);
    }
}
