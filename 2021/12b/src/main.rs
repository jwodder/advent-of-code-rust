use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::Input;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Cave(String);

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Cave {
    fn is_small(&self) -> bool {
        self.0.chars().all(|c| c.is_ascii_lowercase())
    }

    fn is_start(&self) -> bool {
        self.0 == "start"
    }

    fn is_end(&self) -> bool {
        self.0 == "end"
    }
}

struct CaveSystem {
    map: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    fn paths(&self) -> usize {
        let mut ps = vec![Path::new()];
        let mut finished = 0;
        loop {
            let (ended, in_progress): (Vec<_>, Vec<_>) = ps
                .into_iter()
                .flat_map(|p| p.advance(self))
                .partition(Path::is_at_end);
            finished += ended.len();
            ps = in_progress;
            if ps.is_empty() {
                break;
            }
        }
        finished
    }
}

impl FromStr for CaveSystem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<CaveSystem, ParseError> {
        let mut map: HashMap<Cave, Vec<Cave>> = HashMap::new();
        for ln in s.lines() {
            let mut parser = PullParser::new(ln);
            let left = Cave(parser.parse_to::<String, _>('-')?);
            let right = Cave(parser.parse_to::<String, _>(Token::Eof)?);
            map.entry(left.clone()).or_default().push(right.clone());
            map.entry(right).or_default().push(left);
        }
        Ok(CaveSystem { map })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Path {
    path: Vec<Cave>,
    small_seen: HashSet<Cave>,
    smalled_twice: bool,
}

impl Path {
    fn new() -> Path {
        let start = Cave("start".into());
        Path {
            path: vec![start.clone()],
            small_seen: HashSet::from([start]),
            smalled_twice: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.path.last().unwrap().is_end()
    }

    fn move_to(&self, cave: &Cave) -> Path {
        let mut p = self.clone();
        p.path.push(cave.clone());
        if cave.is_small() && !p.small_seen.insert(cave.clone()) {
            p.smalled_twice = true;
        }
        p
    }

    fn can_move_to(&self, cave: &Cave) -> bool {
        !(cave.is_start()
            || (cave.is_small() && self.small_seen.contains(cave) && self.smalled_twice))
    }

    fn advance(&self, system: &CaveSystem) -> Vec<Path> {
        let pos = self.path.last().unwrap();
        // TODO: Try to do this without cloning:
        let next_caves = system.map.get(pos).cloned().unwrap_or_default();
        next_caves
            .into_iter()
            .filter(|c| self.can_move_to(c))
            .map(|c| self.move_to(&c))
            .collect()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for cave in &self.path {
            if !first {
                write!(f, ",")?;
            }
            first = false;
            write!(f, "{}", cave)?;
        }
        Ok(())
    }
}

fn main() {
    let system = Input::from_env().parse::<CaveSystem>();
    println!("{}", system.paths());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let system = concat!(
            "start-A\n",
            "start-b\n",
            "A-c\n",
            "A-b\n",
            "b-d\n",
            "A-end\n",
            "b-end\n",
        )
        .parse::<CaveSystem>()
        .unwrap();
        assert_eq!(system.paths(), 36);
    }

    #[test]
    fn test_example2() {
        let system = concat!(
            "dc-end\n",
            "HN-start\n",
            "start-kj\n",
            "dc-start\n",
            "dc-HN\n",
            "LN-dc\n",
            "HN-end\n",
            "kj-sa\n",
            "kj-HN\n",
            "kj-dc\n",
        )
        .parse::<CaveSystem>()
        .unwrap();
        assert_eq!(system.paths(), 103);
    }

    #[test]
    fn test_example3() {
        let system = concat!(
            "fs-end\n",
            "he-DX\n",
            "fs-he\n",
            "start-DX\n",
            "pj-DX\n",
            "end-zg\n",
            "zg-sl\n",
            "zg-pj\n",
            "pj-he\n",
            "RW-he\n",
            "fs-DX\n",
            "pj-RW\n",
            "zg-RW\n",
            "start-pj\n",
            "he-WI\n",
            "zg-he\n",
            "pj-fs\n",
            "start-RW\n",
        )
        .parse::<CaveSystem>()
        .unwrap();
        assert_eq!(system.paths(), 3509);
    }
}
