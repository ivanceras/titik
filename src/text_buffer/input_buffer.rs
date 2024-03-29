use crate::crossterm::event::{KeyCode, KeyEvent};

/// Input buffer is a 1 dimensional text buffer.
/// It process keystroke and create a string representation
/// depending on each key added to it.
/// If arrow key (ie. left, right) is pressed the cursor location will be changed
/// 1 cell backward/forward with respect to the key being pressed.
/// If backspace stroke is receive, the left most cell relative to the cursor
/// will be remove and all the elements on the right side will be shifted to the left.
///
/// TODO: deal with character that spans more than 1 cell
#[derive(Default, Debug, PartialEq, Clone)]
pub struct InputBuffer {
    content: String,
    cursor_loc: usize,
}

impl InputBuffer {
    /// create a new input buffer
    pub fn new() -> Self {
        InputBuffer {
            content: String::new(),
            cursor_loc: 0,
        }
    }

    /// create an instance of this input buffer with the buffer
    /// content set to value.
    pub fn new_with_value<S: ToString>(value: S) -> Self {
        let value = value.to_string();
        let value_len = value.len();
        InputBuffer {
            content: value,
            cursor_loc: value_len,
        }
    }

    /// return the content of the buffer
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// return the cursor location of the buffer
    pub fn get_cursor_location(&self) -> usize {
        self.cursor_loc
    }

    /// append a character to the buffer and move the cursor location
    /// to the right
    fn add_char(&mut self, c: char) {
        self.content.insert(self.cursor_loc, c);
        self.cursor_loc += 1;
    }

    /// move the cursor location to the left and remove the character
    /// on this new location
    fn backspace(&mut self) {
        if self.content.len() > 0 && self.cursor_loc > 0 {
            self.cursor_loc -= 1;
            self.content.remove(self.cursor_loc);
        }
    }

    /// move the cursor 1 cell to the left
    fn left(&mut self) {
        if self.cursor_loc > 0 {
            self.cursor_loc -= 1;
        }
    }

    /// move the cursor 1 cell to the right
    fn right(&mut self) {
        if self.cursor_loc < self.content.len() {
            self.cursor_loc += 1;
        }
    }

    /// move the cursor location to the start of the buffer
    fn home(&mut self) {
        self.cursor_loc = 0;
    }

    /// move the cursor location to the end of the buffer
    fn end(&mut self) {
        self.cursor_loc = self.content.len();
    }

    /// set the cursor location on this buffer
    pub fn set_cursor_loc(&mut self, x: usize) {
        if x < self.content.len() {
            self.cursor_loc = x;
        }
    }

    /// delete the first character to the right of the cursor
    fn delete(&mut self) {
        if self.cursor_loc < self.content.len() {
            self.content.remove(self.cursor_loc);
        }
    }

    /// Process key events
    ///
    /// Keys to be processed:
    /// - Left
    /// - Right
    /// - Home
    /// - End
    /// - Delete
    /// - Backspace
    /// - Char(char)
    pub fn process_key_event(
        &mut self,
        KeyEvent { code, modifiers: _ }: KeyEvent,
    ) {
        match code {
            KeyCode::Char(c) => {
                self.add_char(c);
            }
            KeyCode::Backspace => {
                self.backspace();
            }
            KeyCode::Left => {
                self.left();
            }
            KeyCode::Right => {
                self.right();
            }
            KeyCode::Home => {
                self.home();
            }
            KeyCode::End => {
                self.end();
            }
            KeyCode::Delete => {
                self.delete();
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_char() {
        let mut input1 = InputBuffer::new();
        input1.add_char('a');
        assert_eq!(String::from("a"), input1.content);
        assert_eq!(input1.cursor_loc, 1);
        input1.add_char('b');
        assert_eq!(String::from("ab"), input1.content);
        assert_eq!(input1.cursor_loc, 2);
        input1.backspace();
        assert_eq!(String::from("a"), input1.content);
        assert_eq!(input1.cursor_loc, 1);
        input1.left(); //move left
        assert_eq!(input1.cursor_loc, 0);
        input1.left(); //move left again, the cursor stays at 0
        assert_eq!(input1.cursor_loc, 0);
        input1.right(); //move right
        assert_eq!(input1.cursor_loc, 1);
        input1.right(); //move right again, the cursor should not be more than the len of the content
        assert_eq!(input1.cursor_loc, 1);

        input1.add_char('b');
        input1.add_char('c');
        assert_eq!("abc", input1.content);
        assert_eq!(3, input1.cursor_loc);
        input1.home(); //press home
        assert_eq!("abc", input1.content); // the string should be the same
        assert_eq!(0, input1.cursor_loc); // the cursor should now be on 0
        input1.end();
        assert_eq!("abc", input1.content); // the string should be the same
        assert_eq!(3, input1.cursor_loc); // the cursor should now be on 3
    }
}
