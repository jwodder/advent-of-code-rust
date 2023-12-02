//! Types for simple string parsing
use std::marker::{Send, Sync};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

/// A string parser that "pulls" or consumes substrings from the start of a
/// string one at a time.
///
/// A `PullParser` is constructed from a string, and the various methods
/// remove an initial portion of the remainder of the string and either return
/// or discard this removed portion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PullParser<'a>(&'a str);

impl<'a> PullParser<'a> {
    /// Construct a `PullParser` for parsing the given string.
    pub fn new(data: &'a str) -> Self {
        PullParser(data)
    }

    /// If the remainder of the string starts with `token`, consume and discard
    /// the matching substring; otherwise, return `Err`.
    ///
    /// If `token` is `Token::Eof`, this is equivalent to calling
    /// [`PullParser::eof()`].
    ///
    /// # Errors
    ///
    /// If the remainder of the string does not start with `token`,
    /// `Err(ParseError::MissingToken(token))` is returned.
    ///
    /// If `token` is `Token::Eof` and the remainder of the string is not
    /// empty, `Err(ParseError::Trailing(remainder))` is returned.
    pub fn skip<T: Into<Token>>(&mut self, token: T) -> Result<(), ParseError> {
        match token.into() {
            Token::Eof => self.eof(),
            token => {
                self.0 = token
                    .consume(self.0)
                    .ok_or(ParseError::MissingToken(token))?;
                Ok(())
            }
        }
    }

    /// Consume the remainder of the string up through the next occurrence of
    /// `token` and return the portion before the token.
    ///
    /// If `token` is `Token::Eof`, consume & return the entirety of the
    /// remainder of the string if it is nonempty; otherwise, return `Err`.
    ///
    /// # Errors
    ///
    /// If `token` does not occur in the remainder of the string (including if
    /// `token` is `Token::Eof` and the remainder is empty),
    /// `Err(ParseError::MissingToken(token))` is returned.
    pub fn scan_to<T: Into<Token>>(&mut self, end: T) -> Result<&'a str, ParseError> {
        let end = end.into();
        let (before, after) = end.split(self.0).ok_or(ParseError::MissingToken(end))?;
        self.0 = after;
        Ok(before)
    }

    /// Like [`PullParser::scan_to()`], but parse the returned string into the
    /// type `U`.
    ///
    /// # Errors
    ///
    /// Errors if [`PullParser::scan_to()`] errors.
    ///
    /// If parsing the string as `U` fails, `Err(ParseError::InvalidParse(e))`
    /// is returned, where `e` contains the `FromStr` error.
    pub fn parse_to<U: FromStr, T: Into<Token>>(&mut self, end: T) -> Result<U, ParseError>
    where
        <U as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        Ok(self
            .scan_to(end)?
            .parse::<U>()
            .map_err(anyhow::Error::new)?)
    }

    /// Split the remainder of the string on each occurrence of the token
    /// `delim`, pass each piece to `parser`, and collect the results.
    ///
    /// If the remainder of the string is empty, an empty `Vec` is returned.
    ///
    /// # Errors
    ///
    /// If any call to `parser` returns an error, the first such error is
    /// returned.
    ///
    /// If a delimiter occurs at the end of the string,
    /// `Err(ParseError::MissingToken(Token::Eof))` is returned.
    pub fn delimited<F, T, U>(mut self, delim: T, mut parser: F) -> Result<Vec<U>, ParseError>
    where
        F: FnMut(&str) -> Result<U, ParseError>,
        T: Into<Token>,
    {
        if self.0.is_empty() {
            return Ok(Vec::new());
        }
        let delim = delim.into();
        let mut elems = Vec::new();
        loop {
            if let Ok(s) = self.scan_to(delim) {
                elems.push(parser(s)?);
            } else {
                elems.push(parser(self.scan_to(Token::Eof)?)?);
                break;
            }
        }
        Ok(elems)
    }

    /// Return `Ok` iff the remainder of the string is empty.
    ///
    /// # Errors
    ///
    /// If the remainder of the string is not empty,
    /// `Err(ParseError::Trailing(remainder))` is returned.
    pub fn eof(&self) -> Result<(), ParseError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ParseError::Trailing(self.0.into()))
        }
    }

    /// Consume the `PullParser` and return the remainder of the string.
    pub fn into_str(self) -> &'a str {
        self.0
    }
}

/// A pattern that can be located in a string
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Token {
    /// A single occurrence of a single character
    Char(char),
    /// A fixed string
    Str(&'static str),
    /// One or more Unicode whitespace characters
    Whitespace,
    /// End of input
    Eof,
}

impl Token {
    /// If `s` starts with the token, return the rest of the string after the
    /// token.  Otherwise, return `None`.
    ///
    /// For `Token::Eof`, return `Some(s)` if `s` is the empty string, `None`
    /// otherwise.
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

    /// Return the portion of `s` before and after the first occurrence of the
    /// token, if any.
    ///
    /// For `Token::Eof`, return `Some((s, ""))` if `s` is not the empty
    /// string, `None` otherwise.
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

/// Error type returned by [`PullParser`] methods and code using [`PullParser`]
#[derive(Debug, Error)]
pub enum ParseError {
    /// Returned by [`PullParser`] methods if a given [`Token`] cannot be
    /// located in the remainder of the parsed string
    #[error("expected token not present in input: {0:?}")]
    MissingToken(
        /// The missing [`Token`]
        Token,
    ),

    /// Returned by [`PullParser::eof()`] and `PullParser::skip(Token::Eof)` if
    /// the remainder of the string is nonempty
    #[error("trailing characters: {0:?}")]
    Trailing(
        /// The nonempty remainder of the string
        String,
    ),

    /// For use by code calling `PullParser`; to be returned if the string
    /// being parsed is syntactically correct but violates some other
    /// requirement
    #[error("input as a whole is invalid: {0:?}")]
    Invalid(
        /// The string being parsed
        String,
    ),

    /// For use by code calling `PullParser`; to be returned if some portion of
    /// the string cannot be parsed
    #[error("invalid token in input: {0:?}")]
    InvalidToken(
        /// The unparsable portion of the string
        String,
    ),

    /// Returned by [`PullParser::parse_to()`] if the parsing of the substring
    /// as type `U` failed.
    #[error("invalid sub-parse: {0:#}")]
    InvalidParse(
        /// The `FromStr` error that occurred
        #[from]
        anyhow::Error,
    ),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> ParseError {
        ParseError::InvalidParse(anyhow::Error::new(e))
    }
}

#[cfg(test)]
mod tests {
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

    fn parse_ints(s: &str) -> Result<Vec<usize>, ParseError> {
        PullParser::new(s).delimited(',', |s| Ok(s.parse::<usize>()?))
    }

    #[rstest]
    #[case("1,2,3", Some(vec![1, 2, 3]))]
    #[case("1,2", Some(vec![1, 2]))]
    #[case("12", Some(vec![12]))]
    #[case("", Some(Vec::new()))]
    #[case("1,2a,3", None)]
    #[case("1,2,", None)]
    #[case(",1,2", None)]
    fn test_parse_ints(#[case] s: &str, #[case] parsed: Option<Vec<usize>>) {
        assert_eq!(parse_ints(s).ok(), parsed);
    }

    #[test]
    fn test_parse_ints_delim_at_end() {
        match parse_ints("1,2,3,") {
            Err(ParseError::MissingToken(Token::Eof)) => (),
            other => panic!("Expected MissingToken(Eof), got {other:?}"),
        }
    }
}
