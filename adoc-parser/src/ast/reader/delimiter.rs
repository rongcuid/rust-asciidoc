use super::block::*;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    /// An open block
    Open,
    /// A non-table block, (delimiter, length)
    NonTable(char, usize),
    /// A table block, (table delimiter, length). Length includes delimiter.
    Table(char, usize),
}

impl Delimiter {
    /// Parse a delimiter. Input must be stripped.
    pub fn parse(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[/=\-.*+_]{4,}$").unwrap();
            // Tables
            static ref RE_T: Regex = Regex::new(r"^[|,:!]={3,}$").unwrap();
        }
        if s == "--" {
            Some(Self::Open)
        } else if RE.is_match(s) {
            Some(Self::NonTable(s.chars().next().unwrap(), s.len()))
        } else if RE_T.is_match(s) {
            Some(Self::Table(s.chars().next().unwrap(), s.len()))
        } else {
            None
        }
    }
    /// Default context and content of delimiter
    pub fn default_context_content(&self) -> (Option<Context>, Option<Content>) {
        match self {
            Delimiter::Open => (Some(Context::Open), Some(Content::Compound)),
            Delimiter::Table(_, _) => (Some(Context::Table), Some(Content::Table)),
            // Comment
            Delimiter::NonTable('/', _) => (None, None),
            // Example
            Delimiter::NonTable('=', _) => (Some(Context::Example), Some(Content::Compound)),
            // Listing
            Delimiter::NonTable('-', _) => (Some(Context::Listing), Some(Content::Verbatim)),
            // Literal
            Delimiter::NonTable('.', _) => (Some(Context::Literal), Some(Content::Verbatim)),
            // Sidebar
            Delimiter::NonTable('*', _) => (Some(Context::Sidebar), Some(Content::Compound)),
            // Passthrough
            Delimiter::NonTable('+', _) => (Some(Context::Passthrough), Some(Content::Raw)),
            // Quote
            Delimiter::NonTable('_', _) => (Some(Context::Quote), Some(Content::Compound)),
            _ => panic!("Delimiter {:?} is impossible", self),
        }
    }
}
