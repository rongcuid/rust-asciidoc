use std::{iter::Peekable, str::Chars};

use super::parser::{Parser, ParserError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementAttrs(pub Vec<ElementAttr>);

impl Parser for ElementAttrs {
    fn parse(s: &str) -> Result<ElementAttrs, ParserError> {
        // TODO: First attribute/shorthand not accounted for; bad errors; line-attr expects right bracket at the end; partial parse results
        let mut attrs = Vec::new();
        let mut scanner = ElementAttrScanner::new(s);
        // If string does not start with left bracket, it is not an attr list
        if scanner.left_bracket().is_none() {
            return Err(ParserError::Mismatch);
        }
        // If first character is a space, this is not an element attr list.
        if scanner.whitespaces().is_some() {
            return Err(ParserError::Mismatch);
        }

        // Parse the remaining of attributes
        loop {
            // Strip whitespaces
            scanner.whitespaces();
            // If encounter right bracket, done
            if scanner.right_bracket().is_some() {
                break;
            }
            // If encounter EOS, error
            if scanner.eos() {
                return Err(ParserError::fail("unexpected end of line"));
            }
            // Expect an attribute
            let attr = ElementAttr::parse(&mut scanner)?;
            // Advance cursor
            scanner.whitespaces();
            scanner.comma();
            attrs.push(attr);
        }
        // Nothing should trail the right bracket
        if scanner.eos() {
            Ok(ElementAttrs(attrs))
        } else {
            Err(ParserError::fail("trailing characters"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use claim::*;

    #[test]
    fn non_attr_returns_none() {
        assert!(ElementAttrs::parse("").unwrap_err().mismatches());
        assert!(ElementAttrs::parse("abc").unwrap_err().mismatches());
        assert!(ElementAttrs::parse("[ ]").unwrap_err().mismatches());
        assert!(ElementAttrs::parse("[ abc]").unwrap_err().mismatches());
        assert!(ElementAttrs::parse("]").unwrap_err().mismatches());
    }

    #[test]
    fn empty_attr_returns_empty_attr_list() {
        let attrs = ElementAttrs::parse("[]").unwrap();
        assert_eq!(attrs, ElementAttrs(vec![]));
    }

    #[test]
    fn positional_attrs_only() {
        let f = ElementAttrs::parse;
        let make_expected = |v: Vec<&str>| {
            ElementAttrs(
                v.iter()
                    .map(|a| ElementAttr::Positional(a.to_string()))
                    .collect(),
            )
        };
        let test = |a, b| {
            println!("Input: {}", a);
            assert_ok_eq!(f(a), make_expected(b))
        };

        // TODO: quickcheck
        test("[pos0]", vec!["pos0"]);
        test("[pos0,pos1]", vec!["pos0", "pos1"]);
        test("[pos0, pos1]", vec!["pos0", "pos1"]);
        test("[pos0 ,pos1]", vec!["pos0", "pos1"]);
        test("[pos0 , pos1]", vec!["pos0", "pos1"]);
        test("[pos0 , pos1 ]", vec!["pos0", "pos1"]);
        test("[pos0   ,   pos1   ]", vec!["pos0", "pos1"]);
        test("[pos0,]", vec!["pos0"]);
        test("[pos0,pos1,]", vec!["pos0", "pos1"]);
        test("[pos0, pos1 ,]", vec!["pos0", "pos1"]);
        test("[pos0 ,pos1,]", vec!["pos0", "pos1"]);
        test("[pos0 , pos1 , ]", vec!["pos0", "pos1"]);
        test("[pos0 , pos1   ,      ]", vec!["pos0", "pos1"]);
        test("[pos0   ,   pos1   ,]", vec!["pos0", "pos1"]);
        test("[\"pos0\"]", vec!["pos0"]);
        test("[\"pos0,pos1\"]", vec!["pos0,pos1"]);
        test("[\"pos0,pos1]\"]", vec!["pos0,pos1]"]);
    }
    // TODO test named attrs and mixed attrs
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementAttr {
    Positional(String),
    Named(String, String),
}

impl ElementAttr {
    /// Assumes cursor is already at first character. Ends with cursor at the last character
    fn parse(scanner: &mut ElementAttrScanner) -> Result<ElementAttr, ParserError> {
        // If starts with a quoted string, then it must be positional
        if let Ok(s) = scanner.quoted_string() {
            return Ok(ElementAttr::Positional(s));
        }
        // Otherwise, take the name string
        let first = scanner.name_string();
        if first.is_empty() {
            return Err(ParserError::fail("attribute is empty"));
        }
        // Look for an equal sign
        scanner.whitespaces();
        let eq = scanner.equal();
        scanner.whitespaces();
        if eq.is_some() {
            // If we encounter an equal sign, expect a named attribute
            scanner.whitespaces();
            let value = scanner.value();
            value
                .map(|v| ElementAttr::Named(first, v))
                .or(Err(ParserError::fail("invalid value")))
        } else {
            // Otherwise, expect positional
            Ok(ElementAttr::Positional(first))
        }
    }
}

/// A scanner with look-ahead of 1. Does not consume unless match.
struct ElementAttrScanner<'a> {
    inner: Peekable<Chars<'a>>,
}

/// Combinators/low-level
impl<'a> ElementAttrScanner<'a> {
    fn new(inner: &'a str) -> Self {
        let inner = inner.chars().peekable();
        Self { inner }
    }
    fn peek(&mut self) -> Option<&char> {
        self.inner.peek()
    }
    fn next(&mut self) -> Option<char> {
        self.inner.next()
    }
    /// Get the next character if it satisfies predicate.
    fn next_if<P: Fn(char) -> bool>(&mut self, pred: P) -> Option<char> {
        let c = self.peek()?;
        if pred(*c) {
            self.next()
        } else {
            None
        }
    }
    /// Get the next character if it matches expected.
    fn next_expect(&mut self, expect: char) -> Option<char> {
        self.next_if(|c| c == expect)
    }
    /// Get the next character if it does not match expected.
    fn next_except(&mut self, except: char) -> Option<char> {
        self.next_if(|c| c != except)
    }
    /// The only supported escaped character is the quote around a quoted value.
    fn escaped_char_or_backslash(&mut self, escaped: char) -> Option<char> {
        let backslash = self.next_expect('\\')?;
        self.next_expect(escaped).or(Some(backslash))
    }
    fn whitespace(&mut self) -> Option<char> {
        self.next_if(char::is_whitespace)
    }
}

/// Character-level
impl<'a> ElementAttrScanner<'a> {
    fn left_bracket(&mut self) -> Option<char> {
        self.next_expect('[')
    }
    fn right_bracket(&mut self) -> Option<char> {
        self.next_expect(']')
    }
    fn comma(&mut self) -> Option<char> {
        self.next_expect(',')
    }
    fn equal(&mut self) -> Option<char> {
        self.next_expect('=')
    }
    fn double_quote(&mut self) -> Option<char> {
        self.next_expect('"')
    }
    fn single_quote(&mut self) -> Option<char> {
        self.next_expect('\'')
    }
    /// A character quoted by the specified quote character.
    fn quoted_char(&mut self, quote: char) -> Option<char> {
        self.escaped_char_or_backslash(quote)
            .or(self.next_except(quote))
    }
    /// A character in an unquoted string
    /// TODO: line-based can use ] too
    fn unquoted_char(&mut self) -> Option<char> {
        self.next_if(|c| !",'\"]".contains(c) && !c.is_whitespace())
    }
    fn name_char(&mut self) -> Option<char> {
        self.next_if(|c| !",'\"]=".contains(c) && !c.is_whitespace())
    }
    /// Consecutive whitespaces are treated as one
    fn whitespaces(&mut self) -> Option<char> {
        let mut some = false;
        while self.whitespace().is_some() {
            some = true;
        }
        if some {
            Some(' ')
        } else {
            None
        }
    }
    /// End of string
    fn eos(&mut self) -> bool {
        self.peek().is_none()
    }
}

/// String level
impl<'a> ElementAttrScanner<'a> {
    fn value(&mut self) -> Result<String, ParserError> {
        self.quoted_string().or(Ok(self.unquoted_string()))
    }
    fn quoted_string(&mut self) -> Result<String, ParserError> {
        if let Some(quote) = self.single_quote().or(self.double_quote()) {
            self.quoted_string_by(quote)
        } else {
            Err(ParserError::Mismatch)
        }
    }
    fn quoted_string_by(&mut self, quote: char) -> Result<String, ParserError> {
        let mut s = String::new();
        loop {
            if let Some(c) = self.quoted_char(quote) {
                s.push(c);
            } else if self.next_expect(quote).is_some() {
                break;
            } else {
                return Err(ParserError::Fail("unexpected end of line".to_owned()));
            }
        }
        Ok(s)
    }
    /// TODO: type level guarantees
    fn unquoted_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.unquoted_char() {
            s.push(c);
        }
        s
    }
    fn name_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.unquoted_char() {
            s.push(c);
        }
        s
    }
}
