use std::str::Lines;

use itertools::{PeekNth, peek_nth};

use super::BlockEvent;

pub struct BlockParser<'input> {
    inner: PeekNth<Lines<'input>>,
}

/// Constructors
impl<'input> BlockParser<'input> {
    pub fn new(source: &'input str) -> Self {
        Self {
            inner: peek_nth(source.lines()),
        }
    }
}

/// Pull-parsing generates `BlockEvent`.
impl<'input> Iterator for BlockParser<'input> {
    type Item = BlockEvent;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}