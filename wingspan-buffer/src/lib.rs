use ropey::Rope;

use druid::Data;

use std::{cmp::min, fmt, io};

#[derive(Clone, Debug, Default, Data)]
pub struct Buffer {
    text: Rope,
    cursur: usize,
}

impl Buffer {
    /// Create a empty `EditableText`.
    pub fn new() -> Self {
        Self {
            text: Rope::new(),
            cursur: 0,
        }
    }

    /// Create a new `EditableText` from a string.
    pub fn new_text(text: &str) -> Self {
        Self {
            text: Rope::from_str(text),
            cursur: 0,
        }
    }

    /// Insert a char at the curser position.
    pub fn insert(&mut self, ch: char) {
        self.text.insert_char(self.cursur, ch);
        self.cursur += 1;
    }

    /// Detete the char behind the curser.
    pub fn delete(&mut self) {
        // This is tricky, as curser and text indecies are subtle
        // Text:      0   1   2   3   4   5
        // Curser: 0    1   2   3   4   5   6

        // If the curser is on 0, we dont delete
        if self.cursur != 0 {
            // Otherwise remove before the curser
            self.text.remove((self.cursur - 1)..self.cursur);
            self.left();
        }
    }

    /// Move the curser to the right
    pub fn right(&mut self) {
        // Make sure we dont go above the text boundry
        self.cursur = min(self.cursur.saturating_add(1), self.text.len_chars())
    }

    /// Move the curser to the left
    pub fn left(&mut self) {
        // Make sure we don't go below index 0.
        self.cursur = self.cursur.saturating_sub(1);
    }

    pub fn rope(&self) -> &Rope {
        &self.text
    }

    pub fn curser(&self) -> usize {
        self.cursur
    }

    pub fn from_reader<T: io::Read>(reader: T) -> io::Result<Self> {
        Ok(Self {
            text: Rope::from_reader(reader)?,
            cursur: 0,
        })
    }
}

impl fmt::Display for Buffer {
    /// Get the text as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omni() {
        let mut text = Buffer::new();
        text.insert('a');
        text.insert('b');
        text.insert('c');
        assert_eq!(&text.to_string(), "abc");
        for _ in 0..10 {
            text.left();
        }
        text.insert('d');
        text.insert('e');
        assert_eq!(&text.to_string(), "deabc");
        text.delete();
        assert_eq!(&text.to_string(), "dabc");
        text.delete();
        assert_eq!(&text.to_string(), "abc");
        text.delete();
        assert_eq!(&text.to_string(), "abc");
        text.delete();
        text.delete();
        text.delete();
        assert_eq!(&text.to_string(), "abc");
        text.right();
        text.right();
        text.insert('f');
        assert_eq!(&text.to_string(), "abfc");
        text.delete();
        assert_eq!(&text.to_string(), "abc");
        text.insert('f');
        text.right();
        text.right();
        text.right();
        text.right();
        text.delete();
        assert_eq!(&text.to_string(), "abf");
        text.delete();
        assert_eq!(&text.to_string(), "ab");
        text.delete();
        assert_eq!(&text.to_string(), "a");
        text.delete();
        assert_eq!(&text.to_string(), "");
    }
}