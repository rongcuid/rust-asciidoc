use std::iter::Peekable;

use crate::ast::reader::{
    self, {Parser, ParserError},
};

use super::{element::ElementAttrs, inline::Inline};

#[derive(Debug, Clone)]
pub struct Block {
    pub attrs: Option<ElementAttrs>,
    pub anchor: Option<String>,
    pub title: Option<Vec<Inline>>,
    pub style: BlockStyle,
    pub context: BlockContext,
    // pub span: Span,
    // TODO delimiters
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockStyle {
    Normal,
    Literal,
    Verse,
    Quote,
    Listing,
    Tip,
    Note,
    Important,
    Warning,
    Caution,
    Abstract,
    PartIntro,
    Comment,
    Example,
    Sidebar,
    Source,
}

impl Default for BlockStyle {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone)]
pub enum BlockContext {
    Admonition(SimpleBlock),
    Audio,
    CalloutList(CompoundBlock),
    DescriptionList(CompoundBlock),
    Example(SimpleBlock),
    FloatingTitle(String),
    Image,
    ListItem(CompoundBlock),
    Listing(VerbatimBlock),
    Literal(VerbatimBlock),
    OrderedList(CompoundBlock),
    Open(BlockContent),
    PageBreak,
    Paragraph(SimpleBlock),
    Passthrough(String),
    Quote(SimpleBlock),
    Section(CompoundBlock),
    Sidebar(SimpleBlock),
    Table(CompoundBlock),
    TableCell(CompoundBlock),
    ThematicBreak,
    UnorderedList(CompoundBlock),
    Verse(SimpleBlock),
}

#[derive(Debug, Clone)]
pub enum BlockContent {
    Compound(CompoundBlock),
    Simple(SimpleBlock),
    Verbatim(VerbatimBlock),
    Raw(RawBlock),
    Empty,
}

#[derive(Debug, Clone)]
pub struct CompoundBlock(Vec<Block>);
#[derive(Debug, Clone)]
pub struct SimpleBlock(Vec<Inline>);
#[derive(Debug, Clone)]
pub struct VerbatimBlock(Vec<Inline>);
#[derive(Debug, Clone)]
pub struct RawBlock(String);

// impl<'a> Element<'a> for Block {
//     /// Assumes already at first line
//     fn parse(s: &mut Peekable<PhysicalLineIter<'a>>) -> Result<Self, ParserError> {
//         // There are multiple possibilities for the first line
//         let first = s.next().expect("expects non-empty first line");
//         // let attrs;
//         // let anchor;
//         // let title;
//         // let style;
//         // let context;
//         // If line looks like an attribute, advance
//         if let Ok(a) = reader::element_attr::ElementAttrs::parse(&first.content) {
//             // attrs = Some(a.into());
//             // We look ahead to see if first line is really an attribute
//             let second = s.peek();
//             if second.is_none() {
//                 // If second line is empty
//             }
//         }
//         // TODO shorthands?

//         todo!()
//         // Ok(Self {
//         //     attrs,
//         //     anchor,
//         //     title,
//         //     style,
//         //     context,
//         // })
//     }
// }
