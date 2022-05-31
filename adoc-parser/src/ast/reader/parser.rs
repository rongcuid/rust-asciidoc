use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParserError {
    /// Definitely not this type
    #[error("definitely not this type")]
    Mismatch,
    /// Expected this type, but failed
    #[error("parse error: {0}")]
    Fail(String),
}

impl ParserError {
    pub fn fail(s: &str) -> Self {
        Self::Fail(s.to_owned())
    }
    pub fn mismatches(&self) -> bool {
        matches!(self, Self::Mismatch)
    }
}

/// A parser that can fail in two ways:
///
/// 1. If it knows input is definitely not `T`, succeeds with `None`.
/// 2. If it expects `T` but fails to parse, fail with Some(Err)
pub trait Parser {
    fn parse(s: &str) -> Result<Self, ParserError>
    where
        Self: Sized;
}
