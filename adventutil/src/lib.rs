pub mod grid;
use either::Either;
use std::fs::File;
use std::io::{read_to_string, stdin, BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Input {
    Stdin,
    File(PathBuf),
}

impl Input {
    pub fn from_env() -> Input {
        std::env::args_os()
            .nth(1)
            .map(|s| Input::File(s.into()))
            .unwrap_or(Input::Stdin)
    }

    pub fn read(self) -> String {
        match self {
            Input::Stdin => read_to_string(stdin().lock()).expect("Error reading stdin"),
            Input::File(path) => read_to_string(BufReader::new(
                File::open(path).expect("Error opening file"),
            ))
            .expect("Error reading file"),
        }
    }

    pub fn lines(self) -> impl Iterator<Item = String> {
        match self {
            Input::Stdin => Either::Left(stdin().lines()),
            Input::File(path) => {
                Either::Right(BufReader::new(File::open(path).expect("Error opening file")).lines())
            }
        }
        .map(|l| l.expect("Error reading input"))
    }

    pub fn parse<T: FromStr>(self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.read().parse::<T>().expect("Error parsing input")
    }
}
