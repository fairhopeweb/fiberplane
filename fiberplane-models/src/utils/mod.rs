use std::borrow::Borrow;

pub mod content_writer;

/// Extension trait that adds methods used for working with Strings
/// and &strs in terms of their UTF-8 characters, as opposed to bytes
pub trait StringExt {
    fn char_count(&self) -> u32;
    fn char_index(&self, offset: u32) -> usize;
    fn char_slice(&self, start: u32, end: u32) -> &str;
    fn char_slice_from(&self, start: u32) -> &str;
}

impl<S> StringExt for S
where
    S: AsRef<str>,
{
    fn char_count(&self) -> u32 {
        char_count(self.as_ref())
    }

    fn char_index(&self, offset: u32) -> usize {
        char_index(self.as_ref(), offset)
    }

    fn char_slice(&self, start: u32, end: u32) -> &str {
        char_slice(self.as_ref(), start, end)
    }

    fn char_slice_from(&self, start: u32) -> &str {
        char_slice_from(self.as_ref(), start)
    }
}

/// Counts the number of (USV) characters in a string.
pub fn char_count<T>(text: &T) -> u32
where
    T: Borrow<str> + ?Sized,
{
    text.borrow().chars().count() as u32
}

/// Calculates the byte index at which the character with the given offset starts.
pub fn char_index(text: &str, offset: u32) -> usize {
    text.chars()
        .take(offset as usize)
        .map(|c| c.len_utf8())
        .sum()
}

/// Returns a slice of a string, based on character offsets instead of byte
/// indices.
pub fn char_slice(text: &str, start: u32, end: u32) -> &str {
    let start = char_index(text, start);
    let end = char_index(text, end);
    &text[start..end]
}

/// Returns a slice of a string, based on character offsets instead of byte
/// indices.
pub fn char_slice_from(text: &str, start: u32) -> &str {
    let start = char_index(text, start);
    &text[start..]
}
