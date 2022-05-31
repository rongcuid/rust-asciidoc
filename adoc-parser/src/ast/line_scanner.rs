use std::str::Lines;

use itertools::{MultiPeek, PeekNth};

use super::{
    element::{ElementAttr, ElementAttrs},
    reader::{self, Delimiter, Parser, ParserError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PhysicalLine<'a> {
    EOF,
    /// An empty line contains nothing or only whitespace
    Empty,
    Line(&'a str),
}

impl<'a> From<Option<&'a str>> for PhysicalLine<'a> {
    fn from(s: Option<&'a str>) -> Self {
        match s {
            None => PhysicalLine::EOF,
            Some(s) => {
                if s.trim().is_empty() {
                    PhysicalLine::Empty
                } else {
                    PhysicalLine::Line(s)
                }
            }
        }
    }
}

impl<'a> PhysicalLine<'a> {
    /// If EOF or empty, return a mismatch. Otherwise, apply `f`
    fn flat_map_line<F, B>(self, f: F) -> Result<B, ParserError>
    where
        F: Fn(&'a str) -> Result<B, ParserError>,
    {
        match self {
            PhysicalLine::EOF | PhysicalLine::Empty => Err(ParserError::Mismatch),
            PhysicalLine::Line(s) => f(s),
        }
    }
    /// If EOF or empty, return a mismatch. Otherwise, apply `f` and wrap in `Ok`
    fn map_line<F, B>(self, f: F) -> Result<B, ParserError>
    where
        F: Fn(&'a str) -> B,
    {
        match self {
            PhysicalLine::EOF | PhysicalLine::Empty => Err(ParserError::Mismatch),
            PhysicalLine::Line(s) => Ok(f(s)),
        }
    }
}

/// A LL(N) scanner which operates on lines
pub struct LineScanner<'a> {
    inner: PeekNth<Lines<'a>>,
}

/// Token iterators
impl<'a> LineScanner<'a> {
    /// Returns one physical line
    fn next(&mut self) -> PhysicalLine {
        self.inner.next().into()
    }
    fn peek(&mut self) -> PhysicalLine {
        self.inner.peek().map(|s| *s).into()
    }
    fn peek_nth(&mut self, n: usize) -> PhysicalLine {
        self.inner.peek_nth(n).map(|s| *s).into()
    }
    fn advance_if_ok<T>(&mut self, x: Result<T, ParserError>) -> Result<T, ParserError> {
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
    pub fn peek_element_attrs(&mut self) -> Result<ElementAttrs, ParserError> {
        self.peek()
            .flat_map_line(|line| reader::ElementAttrs::parse(line).map(ElementAttrs::from))
    }
    pub fn peek_delimiter(&mut self) -> Result<Delimiter, ParserError> {
        self.peek()
            .flat_map_line(|line| Delimiter::parse(line).ok_or(ParserError::Mismatch))
    }
    pub fn peek_non_empty_line(&mut self) -> Result<String, ParserError> {
        self.peek().map_line(str::to_owned)
    }
    pub fn peek_empty(&mut self) -> bool {
        let next = self.peek();
        if matches!(next, PhysicalLine::Empty) {
            true
        } else {
            false
        }
    }
    pub fn peek_eof(&mut self) -> bool {
        let next = self.peek();
        if matches!(next, PhysicalLine::EOF) {
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
                PhysicalLine::Line(line) => lines.push(line.to_owned()),
                PhysicalLine::EOF | PhysicalLine::Empty => break,
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
