use ropey::Rope;

use druid::Data;

use std::cell::Cell;
use std::{cmp::min, fmt, io};

#[derive(Clone, Debug, Default)]
pub struct Buffer {
    text: Rope,
    cursor: Cell<usize>,
}

impl Data for Buffer {
    fn same(&self, other: &Self) -> bool {
        // Can't yet be derived
        // https://xi.zulipchat.com/#narrow/stream/147926-druid/topic/impl.20Data.20for.20Cell/near/228083891
        self.text.same(&other.text) && self.cursor == other.cursor
    }
}

impl Buffer {
    /// Create a empty `EditableText`.
    pub fn new() -> Self {
        Self {
            text: Rope::new(),
            cursor: Cell::new(0),
        }
    }

    /// Create a new `EditableText` from a string.
    pub fn new_text(text: &str) -> Self {
        Self {
            text: Rope::from_str(text),
            cursor: Cell::new(0),
        }
    }

    /// Insert a char at the curser position.
    pub fn insert(&mut self, ch: char) {
        self.text.insert_char(self.cursor.get(), ch);
        *self.cursor.get_mut() += 1;
    }

    /// Detete the char behind the curser.
    pub fn delete(&mut self) {
        // This is tricky, as curser and text indecies are subtle
        // Text:      0   1   2   3   4   5
        // Curser: 0    1   2   3   4   5   6

        // If the curser is on 0, we dont delete
        if self.cursor.get() != 0 {
            // Otherwise remove before the curser
            let c = self.cursor.get();

            self.text.remove((c - 1)..c);
            self.left();
        }
    }

    /// Move the curser to the right
    pub fn right(&mut self) {
        // Make sure we dont go above the text boundry
        self.cursor.set(min(
            self.cursor.get().saturating_add(1),
            self.text.len_chars(),
        ))
    }

    /// Move the curser to the left
    pub fn left(&mut self) {
        // Make sure we don't go below index 0.
        self.cursor.set(self.cursor.get().saturating_sub(1));
    }

    pub fn up(&mut self) {
        // TODO: Test + edge cases
        let lineno = self.rope().char_to_line(self.cursor.get());
        let start_of_line = self.rope().line_to_char(lineno);
        let into_this_line = self.cursor.get() - start_of_line;

        // TODO: Case where we need to move back the curser
        let new_curser =
            self.rope().line_to_char(lineno.saturating_sub(1)) + into_this_line;
        self.cursor.set(new_curser);
    }

    pub fn down(&mut self) {
        // TODO: Test + edge cases, merge_with_up
        let lineno = self.rope().char_to_line(self.cursor.get());
        let start_of_line = self.rope().line_to_char(lineno);
        let into_this_line = self.cursor.get() - start_of_line;

        let new_curser = self.rope().line_to_char(lineno + 1) + into_this_line;
        self.cursor.set(new_curser);
    }

    pub fn rope(&self) -> &Rope {
        &self.text
    }

    pub fn cursor(&self) -> usize {
        self.cursor.get()
    }

    pub fn set_cursor(&self, curser: usize) {
        // Text:      0   1   2   3   4   5
        // Curser: 0    1   2   3   4   5   6
        assert!(curser <= self.text.len_chars() + 1);
        self.cursor.set(curser);
    }

    pub fn from_reader<T: io::Read>(reader: T) -> io::Result<Self> {
        Ok(Self {
            text: Rope::from_reader(reader)?,
            cursor: Cell::new(0),
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
