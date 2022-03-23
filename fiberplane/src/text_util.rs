use std::borrow::Borrow;

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
pub fn char_slice(text: &str, start: usize, end: usize) -> &str {
    let start = char_index(text, start as u32);
    let end = char_index(text, end as u32);
    &text[start..end]
}
