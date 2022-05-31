use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct DocumentAttrName(String);

impl DocumentAttrName {
    pub fn parse(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^:([a-zA-Z0-9_\-]+):").unwrap();
        }
        RE.captures(s)
            .map(|caps| Self(caps[1].to_ascii_lowercase().to_owned()))
    }
}

#[derive(Debug, Clone)]
pub enum DocumentAttrValue {
    ValueLine(String),
    ValuePartialLine(String),
    ValuePartialLineHardBreak(String),
}

impl DocumentAttrValue {
    pub fn parse(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE_HB: Regex = Regex::new(r"(.*)\s+\+\s+\\$").unwrap();
            static ref RE_LB: Regex = Regex::new(r"(.*)\s+\\$").unwrap();
        }
        if let Some(caps) = RE_HB.captures(s) {
            Some(Self::ValuePartialLineHardBreak(caps[1].to_owned()))
        } else if let Some(caps) = RE_LB.captures(s) {
            Some(Self::ValuePartialLine(caps[1].to_owned()))
        } else if s.len() != 0 {
            Some(Self::ValueLine(s.to_owned()))
        } else {
            None
        }
    }
}
