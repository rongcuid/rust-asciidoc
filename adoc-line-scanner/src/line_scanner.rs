use std::str::Lines;

use itertools::PeekNth;

use crate::{asciidoc_line::AsciidocLine, error::LineScannerError};

/// A LL(N) scanner which operates on lines
pub struct LineScanner<'a> {
    inner: PeekNth<Lines<'a>>,
}

/// Token iterators
impl<'a> LineScanner<'a> {
    /// Returns one physical line
    fn next(&mut self) -> AsciidocLine {
        self.inner.next().into()
    }
    fn peek(&mut self) -> AsciidocLine {
        self.inner.peek().map(|s| *s).into()
    }
    fn peek_nth(&mut self, n: usize) -> AsciidocLine {
        self.inner.peek_nth(n).map(|s| *s).into()
    }
    fn advance_if_ok<T>(&mut self, x: Result<T, LineScannerError>) -> Result<T, LineScannerError> {
        if x.is_ok() {
            self.next();
        }
        x
    }
    fn advance(&mut self) {
        self.next();
    }
    fn advance_by(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }
}

/// Scanner commands for single (logical) lines
impl<'a> LineScanner<'a> {
    // pub fn peek_element_attrs(&mut self) -> Result<ElementAttrs, ParserError> {
    //     self.peek()
    //         .flat_map_line(|line| reader::ElementAttrs::parse(line).map(ElementAttrs::from))
    // }
    // pub fn peek_delimiter(&mut self) -> Result<Delimiter, ParserError> {
    //     self.peek()
    //         .flat_map_line(|line| Delimiter::parse(line).ok_or(ParserError::Mismatch))
    // }
    // pub fn peek_non_empty_line(&mut self) -> Result<String, ParserError> {
    //     self.peek().map_line(str::to_owned)
    // }
    pub fn peek_empty(&mut self) -> bool {
        let next = self.peek();
        if matches!(next, AsciidocLine::Empty) {
            true
        } else {
            false
        }
    }
    pub fn peek_eof(&mut self) -> bool {
        let next = self.peek();
        if matches!(next, AsciidocLine::EOF) {
            true
        } else {
            false
        }
    }
}

/// Scanner command for multiple lines
impl<'a> LineScanner<'a> {
    /// Scan a block of contiguous empty lines. Returns whether there are any empty lines
    pub fn empty_lines(&mut self) -> bool {
        let mut empty = false;
        while self.peek_empty() {
            self.advance();
            empty = true;
        }
        empty
    }
    /// Scan a block of contiguous lines. Never fails.
    pub fn contiguous_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        loop {
            let line = self.peek();
            match line {
                AsciidocLine::RegularLine(line) => lines.push(line.to_owned()),
                AsciidocLine::EOF | AsciidocLine::Empty => break,
            }
            self.advance();
        }
        lines
    }
    pub fn delimited_block(&mut self) -> Vec<String> {
        todo!()
    }
    /// A block where every line is prefixed by `prefix`. 
    /// 
    /// For instance, an indented block.
    pub fn prefixed_block(&mut self, prefix: &str) -> Vec<String> {
        todo!()
    }
}
