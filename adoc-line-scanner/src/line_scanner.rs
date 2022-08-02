use std::str::{CharIndices, Lines};

use itertools::{peek_nth, PeekNth};

/// A LL(N) scanner which operates on lines.
/// At the beginning of any command, the cursor _MUST_ be at the beginning of a line.
/// 
/// Double references are avoided
pub struct LineScanner<'a> {
    inner: PeekNth<Lines<'a>>,
}

/// Basic utilities
impl<'a> LineScanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: peek_nth(source.lines())
        }
    }
}

impl<'a> LineScanner<'a> {
}

/// Commands used by next-level parser/scanner
impl<'a> LineScanner<'a> {
    /// Cursor at a line containing only spaces. 
    /// Does not match EOF.
    pub fn peek_empty(&mut self) -> bool {
        let all_ws = self.inner.peek().map(|&line| {
            line.chars().all(char::is_whitespace)
        });
        matches!(all_ws, Some(true))
    }
    /// Cursor is at EOF
    pub fn peek_eof(&mut self) -> bool {
        self.inner.peek().is_none()
    }
    /// Get one line, unprocessed
    pub fn peek(&mut self) -> Option<&str> {
        self.inner.peek().cloned()
    }
}
