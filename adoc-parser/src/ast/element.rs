/// This module holds data structure for the final AST.
/// Intermediate syntax constructs and the parser are not included.
use std::{collections::HashMap, iter::Peekable};

use super::reader::ParserError;

use super::reader;

/// Documents, Blocks, and Inline are all elements
// pub trait Element<'a> {
//     /// TODO: better error handling
//     fn parse(lines: &mut Peekable<PhysicalLineIter<'a>>) -> Result<Self, ParserError>
//     where
//         Self: Sized;
// }

/// Attribute map for elements
#[derive(Debug, Clone)]
pub struct ElementAttrs {
    by_name: HashMap<ElementName, ElementAttr>,
    by_index: Vec<ElementAttr>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ElementName(String);

#[derive(Debug, Clone)]
pub struct ElementAttr(String);

impl From<reader::ElementAttrs> for ElementAttrs {
    fn from(attrs: reader::ElementAttrs) -> Self {
        let mut by_name = HashMap::new();
        let mut by_index = Vec::new();
        for attr in attrs.0 {
            match attr {
                super::reader::ElementAttr::Positional(a) => {
                    by_index.push(a);
                }
                super::reader::ElementAttr::Named(n, v) => {
                    by_name.insert(ElementName(n), ElementAttr(v));
                }
            }
        }
        todo!()
    }
}
