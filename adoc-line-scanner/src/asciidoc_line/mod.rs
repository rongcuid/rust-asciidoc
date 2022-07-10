mod block_delimiter;

use crate::error::LineScannerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsciidocLine<'a> {
    EOF,
    /// An line containing nothing or only whitespace
    Empty,
    /// A line without context info
    Line(&'a str),
}

impl<'a> From<Option<&'a str>> for AsciidocLine<'a> {
    fn from(s: Option<&'a str>) -> Self {
        match s {
            None => AsciidocLine::EOF,
            Some(s) => {
                if s.trim().is_empty() {
                    AsciidocLine::Empty
                } else {
                    AsciidocLine::Line(s)
                }
            }
        }
    }
}

impl<'a> AsciidocLine<'a> {
    /// If EOF or empty, return a mismatch. Otherwise, apply `f`
    fn flat_map_line<F, B>(self, f: F) -> Result<B, LineScannerError>
    where
        F: Fn(&'a str) -> Result<B, LineScannerError>,
    {
        match self {
            AsciidocLine::EOF | AsciidocLine::Empty => Err(LineScannerError::Mismatch),
            AsciidocLine::Line(s) => f(s),
        }
    }
    /// If EOF or empty, return a mismatch. Otherwise, apply `f` and wrap in `Ok`
    fn map_line<F, B>(self, f: F) -> Result<B, LineScannerError>
    where
        F: Fn(&'a str) -> B,
    {
        match self {
            AsciidocLine::EOF | AsciidocLine::Empty => Err(LineScannerError::Mismatch),
            AsciidocLine::Line(s) => Ok(f(s)),
        }
    }
}