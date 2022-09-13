use lazy_static::lazy_static;
use regex::Regex;

use crate::multiparse::MultiParse;

/// Line level token in document header.
/// Only every produces the "head" types: i.e. only first wrapped lines are produced
pub(crate) enum HeaderLineToken {
    DocumentTitle(String),
    DocumentAttr {
        attr: String,
        unset: bool,
        /// Unparsed
        value: Option<String>,
        wrap: DocumentAttrWrap,
    },
}

pub(crate) enum HeaderLineTokenType {}

impl MultiParse for HeaderLineToken {
    fn multiparse(s: &str) -> Vec<Self> {
        lazy_static! {
            static ref RE_TITLE: Regex = Regex::new(r"^=\s(.*)$").unwrap();
            static ref RE_ATTR: Regex =
                Regex::new(r"^:(?P<unset-1>!?)(?P<attr>\w[-\w])(?P<unset-2>!?):(?P<value>\s+.+)?(?P<wrap>(\s\+)?(\s\\))?$")
                    .unwrap();
        }
        let mut possibilities = Vec::new();
        // Document title
        if let Some(cap) = RE_TITLE.captures(s) {
            possibilities.push(Self::DocumentTitle(cap[1].to_owned()));
        }
        // Document attribute
        if let Some(cap) = RE_ATTR.captures(s) {
            let attr = cap["attr"].to_owned();
            let unset = cap.name("unset-1").is_some() || cap.name("unset-2").is_some();
            let value = cap.name("value").map(|x| x.as_str().to_owned());
            let wrap = if let Some(w) = cap.name("wrap") {
                if w.as_str().contains("+") {
                    DocumentAttrWrap::HardWrap
                } else {
                    DocumentAttrWrap::SoftWrap
                }
            } else {
                DocumentAttrWrap::NoWrap
            };
            possibilities.push(Self::DocumentAttr {
                attr,
                unset,
                value,
                wrap,
            });
        }
        possibilities
    }
}

pub enum DocumentAttrWrap {
    NoWrap,
    SoftWrap,
    HardWrap,
}

/// Line level token, with unparsed source
pub(crate) struct LineToken {
    ty: LineTokenType,
    inner: String,
}

pub(crate) enum LineTokenType {
    /// `****` would have delimiter `*` and level 4; `--` has delimiter `-` and level 2
    Delimiter {
        delimiter: char,
        level: u64,
    },
    /// `|===` would have delimiter `|` and level 3
    TableDelimiter {
        delimiter: char,
        level: u64,
    },
    ParagraphLine,
    ParagraphLineHardBreak,
    BlockAttr,
}
