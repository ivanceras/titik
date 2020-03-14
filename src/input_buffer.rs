use crossterm::event::{
    KeyCode,
    KeyEvent,
};

/// Input buffer is a one dimensional text buffer.
/// It process keystroke and create a string representation
/// depending on each key added to it.
/// If arrow key (ie. left, right) is pressed the cursor location will be changed
/// 1 cell backward/forward with respect to the key being pressed.
/// If backspace stroke is receive, the left most cell relative to the cursor
/// will be remove and all the elements on the right side will be shifted to the left.
///
/// TODO: deal with character that spans more than 1 cell
#[derive(Default, Debug, PartialEq)]
pub struct InputBuffer {
    content: String,
    cursor_loc: usize,
}

impl InputBuffer {
    pub fn new() -> Self {
        InputBuffer {
            content: String::new(),
            cursor_loc: 0,
        }
    }

    pub fn new_with_value<S: ToString>(value: S) -> Self {
        let value = value.to_string();
        let value_len = value.len();
        InputBuffer {
            content: value,
            cursor_loc: value_len,
        }
    }

    /// replace the value of the input buffer with `value`
    /// the cursor_location is also set to the end of the buffer.
    pub fn set_value<S: ToString>(&mut self, value: S) {
        self.content = value.to_string();
        self.cursor_loc = self.content.len();
    }

    /// return the content of the buffer
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// return the cursor location of the buffer
    pub fn get_cursor_location(&self) -> usize {
        self.cursor_loc
    }

    fn add_char(&mut self, c: char) {
        self.content.insert(self.cursor_loc, c);
        self.cursor_loc += 1;
    }

    fn backspace(&mut self) {
        if self.content.len() > 0 && self.cursor_loc > 0 {
            self.cursor_loc -= 1;
            self.content.remove(self.cursor_loc);
        }
    }

    fn left(&mut self) {
        if self.cursor_loc > 0 {
            self.cursor_loc -= 1;
        }
    }

    fn right(&mut self) {
        if self.cursor_loc < self.content.len() {
            self.cursor_loc += 1;
        }
    }

    fn home(&mut self) {
        self.cursor_loc = 0;
    }

    fn end(&mut self) {
        self.cursor_loc = self.content.len();
    }

    // Keys to be processed:
    // - Left
    // - Right
    // - Home
    // - End
    // - Delete
    // - Backspace
    // - Char(char)
    pub fn process_key_event(
        &mut self,
        KeyEvent { code, modifiers }: KeyEvent,
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
