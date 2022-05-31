use lazy_static::lazy_static;
use regex::Regex;

use super::parser::{Parser, ParserError};

/// A section header with level
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionTitle {
    pub marker: char,
    pub level: usize,
    pub title: String,
}

impl Parser for SectionTitle {
    fn parse(s: &str) -> Result<Self, ParserError> {
        lazy_static! {
            // This matches ADoc-style and MD-style
            static ref RE: Regex = Regex::new(r"^(={1,6}|#{1,6})\s+(.+)$").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let marker = caps[1].chars().next().unwrap();
            let level = caps[1].len() - 1;
            let title = caps[2].trim_end().to_owned();
            Ok(SectionTitle {
                marker,
                level,
                title,
            })
        } else {
            Err(ParserError::Mismatch)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use claim::*;
    #[test]
    fn not_a_section_title() {
        let test = |s| assert_eq!(SectionTitle::parse(s), Err(ParserError::Mismatch));

        test("");
        test("=");
        test("= ");
        test(" =");
        test("#");
        test("# ");
        test(" #");
        test("akdslfu9q3w8(*&)IJOKLJALXLf][]");
    }

    #[test]
    fn valid_section_title() {
        let test = |marker: &str, level, title: &str| {
            let marker = marker.repeat(level + 1);
            assert_ok_eq!(
                SectionTitle::parse(&vec![&marker[..], title].join(" ")),
                SectionTitle {
                    marker: marker.chars().next().unwrap(),
                    level,
                    title: title.trim().to_owned(),
                }
            );
        };
        let test_all = |title| {
            for marker in vec!["#", "="] {
                for level in 0..5 {
                    test(marker, level, title);
                }
            }
        };
        test_all("foo");
        test_all(" foo");
        test_all(" foo ");
        test_all("foo ");
        test_all(" =foo");
        test_all(" = foo");
        test_all("= foo");
    }
}
