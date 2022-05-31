use std::iter::Peekable;

use super::reader::*;

use super::element::ElementAttrs;

#[derive(Debug, Clone)]
pub struct Inline {
    pub attrs: ElementAttrs,
    pub kind: InlineKind,
}

#[derive(Debug, Clone)]
pub enum InlineKind {
    /// Leaf kind
    Unquoted,
    /// Emphasis, strong, etc
    Quoted {
        ty: QuoteType,
        content: Vec<Inline>,
    },
    Other {
        ty: String,
        content: Vec<Inline>,
    },
}

/// Inline quotes, precedence in this order. Each quote may have unconstrained/constrained form.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum QuoteType {
    /// `**strong**` and `*strong*`
    Strong,
    /// `"\`double-quoted\`"`, constrained
    Double,
    /// `'\`single-quoted\`'`, constrained
    Single,
    /// `\`\`monospaced\'\`` and `\`monospaced\``
    Monospaced,
    /// `__emphasis__` and `_emphasis_`
    Emphasis,
    /// `##mark##` and `#mark#`
    Mark,
    /// `^superscript^`, unconstrained
    Superscript,
    /// `~subscript~`, unconstrained
    Subscript,
}

// impl<'a> Element<'a> for Inline {
//     fn parse(s: &mut Peekable<PhysicalLineIter<'a>>) -> Result<Self, ParserError> {
//         todo!()
//     }
// }
