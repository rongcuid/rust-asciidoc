mod block_delimiter;

use self::block_delimiter::BlockDelimiter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsciidocLine {
    EOF,
    /// An line containing nothing or only whitespace
    Empty,
    // DocAttribute(Attr),
    /// A block attribute
    // BlockAttribute(Vec<Attr>),
    Delimiter(BlockDelimiter),
    /// A line with regular substitutions
    RegularLine(String),
    /// A line that is not processed
    PassthroughLine(String),
    CommentLine(String),
}