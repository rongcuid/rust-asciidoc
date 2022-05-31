use anyhow::*;
use std::result::Result::{Err, Ok};

use std::string::ParseError;
use std::{collections::HashMap, iter::Peekable};

use crate::ast::reader::ParserError;
use crate::ast::reader::{
    Parser, SectionTitle,
};

use super::{block::Block, inline::Inline};

#[derive(Debug, Clone)]
pub struct DocumentAttrs {
    by_name: HashMap<String, Vec<Inline>>,
}

// The AST below

#[derive(Clone, Debug, Default)]
pub struct Document {
    pub title: Option<String>,
    // FIXME placeholder
    pub attrs: Option<DocumentAttrs>,
    pub blocks: Vec<Block>,
}

// impl<'a> Element<'a> for Document {
//     fn parse(s: &mut Peekable<PhysicalLineIter<'a>>) -> Result<Self, ParserError> {
//         // The first line is usually a title
//         let first = s.next();
//         // Empty document
//         if first.is_none() {
//             return Ok(Default::default());
//         }
//         let first = first.unwrap();
//         let st = SectionTitle::parse(&first.content);
//         let title;
//         match st {
//             Ok(st) => {
//                 // TODO: warnings, etc
//                 // We found a section title
//                 title = Some(st.title);
//             }
//             Err(e) => {
//                 if e.mismatches() {
//                     title = None;
//                 } else {
//                     return Err(ParserError::fail("section title parse error"));
//                 }
//             }
//         }
//         // FIXME: author
//         // FIXME: revision
//         // FIXME: actually parse attributes
//         Self::skip_attributes(s);

//         let blocks = Self::parse_blocks(s)?;

//         Ok(Self {
//             title,
//             attrs: None,
//             blocks,
//         })
//     }
// }

// impl<'a> Document {
//     /// FIXME: actually parse attributes
//     fn skip_attributes(s: &mut Peekable<PhysicalLineIter<'a>>) {
//         // Consume until encountering an empty line
//         while Self::empty_line(s) {}
//     }

//     fn skip_empty_lines(s: &mut Peekable<PhysicalLineIter<'a>>) {
//         // Consume until encountering an empty line
//         while Self::empty_line(s) {}
//     }
//     // Consume an empty line
//     fn empty_line(s: &mut Peekable<PhysicalLineIter<'a>>) -> bool {
//         if let Some(line) = s.peek() {
//             if line.content.trim().is_empty() {
//                 s.next();
//                 return true;
//             }
//         }
//         false
//     }

//     fn parse_blocks(s: &mut Peekable<PhysicalLineIter<'a>>) -> Result<Vec<Block>, ParserError> {
//         let mut blocks = Vec::new();
//         Self::skip_empty_lines(s);
//         loop {
//             // Loop until EOF
//             if s.peek().is_none() {
//                 break;
//             }
//             let block = Block::parse(s);
//             match block {
//                 Ok(block) => blocks.push(block),
//                 Err(e) => {
//                     // TODO: partial output
//                 }
//             }
//         }
//         Ok(blocks)
//     }
// }
