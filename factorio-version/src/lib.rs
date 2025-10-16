use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct Version([u16; 3]);

impl From<[u16; 3]> for Version {
    fn from(value: [u16; 3]) -> Self {
        Self(value)
    }
}

impl From<Version> for [u16; 3] {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0[0], self.0[1], self.0[2])
    }
}

#[derive(Debug)]
pub enum ParseError {
    ParseInt(ParseIntError),
    Missing,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;
        match self {
            ParseInt(e) => {
                write!(f, "invalid version part: {e}")
            }
            Missing => {
                write!(f, "incomplete version")
            }
        }
    }
}

impl Error for ParseError {}

impl FromStr for Version {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .splitn(3, '.')
            .map(|part| part.parse().map_err(ParseError::ParseInt));

        let mut next = || iter.next().ok_or(ParseError::Missing).flatten();

        Ok(Self([next()?, next()?, next()?]))
    }
}
