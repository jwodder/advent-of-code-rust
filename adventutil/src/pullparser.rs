use std::marker::{Send, Sync};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PullParser<'a> {
    data: &'a str,
}

impl<'a> PullParser<'a> {
    pub fn new(data: &'a str) -> Self {
        PullParser { data }
    }

    pub fn skip<T: Into<Token>>(&mut self, token: T) -> Result<(), ParseError> {
        match token.into() {
            Token::Eof => self.eof(),
            token => {
                self.data = token
                    .consume(self.data)
                    .ok_or(ParseError::MissingToken(token))?;
                Ok(())
            }
        }
    }

    pub fn scan_to<T: Into<Token>>(&mut self, end: T) -> Result<&'a str, ParseError> {
        let end = end.into();
        let (before, after) = end.split(self.data).ok_or(ParseError::MissingToken(end))?;
        self.data = after;
        Ok(before)
    }

    pub fn parse_to<U: FromStr, T: Into<Token>>(&mut self, end: T) -> Result<U, ParseError>
    where
        <U as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        Ok(self
            .scan_to(end)?
            .parse::<U>()
            .map_err(anyhow::Error::new)?)
    }

    pub fn eof(&self) -> Result<(), ParseError> {
        if self.data.is_empty() {
            Ok(())
        } else {
            Err(ParseError::Trailing(self.data.into()))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Token {
    Char(char),
    Str(&'static str),
    /// One or more Unicode whitespace characters
    Whitespace,
    Eof,
}

impl Token {
    pub fn consume<'a>(&self, data: &'a str) -> Option<&'a str> {
        match self {
            Token::Char(c) => data.strip_prefix(*c),
            Token::Str(s) => data.strip_prefix(s),
            Token::Whitespace => {
                let s = data.trim_start();
                (s != data).then_some(s)
            }
            Token::Eof => data.is_empty().then_some(data),
        }
    }

    pub fn split<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)> {
        match self {
            Token::Char(c) => s.split_once(*c),
            Token::Str(t) => s.split_once(t),
            Token::Whitespace => {
                let (before, after) = s.split_once(char::is_whitespace)?;
                Some((before, after.trim_start()))
            }
            Token::Eof => (!s.is_empty()).then_some((s, "")),
        }
    }
}

impl From<char> for Token {
    fn from(c: char) -> Token {
        Token::Char(c)
    }
}

impl From<&'static str> for Token {
    fn from(s: &'static str) -> Token {
        Token::Str(s)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected token not present in input: {0:?}")]
    MissingToken(Token),
    #[error("Trailing characters: {0:?}")]
    Trailing(String),
    #[error("Input as a whole is invalid: {0:?}")]
    Invalid(String),
    #[error("Invalid token in input: {0:?}")]
    InvalidToken(String),
    #[error("Invalid sub-parse: {0:#}")]
    InvalidParse(#[from] anyhow::Error),
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    fn parse_coord(s: &str) -> Result<(usize, usize), ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip('(')?;
        let x = parser.parse_to::<usize, _>(',')?;
        parser.skip(Token::Whitespace)?;
        let y = parser.parse_to::<usize, _>(')')?;
        parser.eof()?;
        Ok((x, y))
    }

    #[rstest]
    #[case("(2, 3)", Some((2, 3)))]
    #[case("(2,  3)", Some((2, 3)))]
    #[case(" (2, 3)", None)]
    #[case("( 2, 3)", None)]
    #[case("(-2, 3)", None)]
    #[case("(2 3)", None)]
    #[case("(2, 3", None)]
    #[case("(2, 3) ", None)]
    #[case("(2,3)", None)]
    #[case("2, 3)", None)]
    fn test_parse_coord(#[case] s: &str, #[case] parsed: Option<(usize, usize)>) {
        assert_eq!(parse_coord(s).ok(), parsed);
    }
}
