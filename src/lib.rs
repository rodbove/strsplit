//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else if remainder.len() > 0 {
            self.remainder.take()
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + 1))
    }
}

/// Returns all characters on string up to the char informed
///
/// # Arguments
///
/// * `s` - A string to be handled
/// * `c` - Character used as limit for the string to be returned
///
/// # Examples
pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell")
}

#[test]
fn split_test() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail_test() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d"]);
}

#[test]
fn empty_test() {
    let haystack = "a b  c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "", "c", "d"]);
}
