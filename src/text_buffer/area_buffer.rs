use crate::crossterm::event::{KeyCode, KeyEvent};
use glam::*;
use unicode_width::UnicodeWidthChar;

/// Area buffer is a 2 dimensional text buffer
#[derive(Default, Debug, PartialEq, Clone)]
pub(crate) struct AreaBuffer {
    pub(crate) content: Vec<Vec<char>>,
    content_width: usize,
    cursor_loc_x: usize,
    cursor_loc_y: usize,
}

impl AreaBuffer {
    fn add_char(&mut self, c: char) {
        let line = self
            .content
            .get_mut(self.cursor_loc_y)
            .expect("must have a line");
        line.insert(self.cursor_loc_x, c);
        self.cursor_loc_x += 1;
        self.calc_content_width();
    }

    fn calc_content_width(&mut self) {
        self.content_width = self
            .content
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);
    }

    pub(crate) fn add_line<S: ToString>(&mut self, s: S) {
        let line = s.to_string().chars().collect();
        self.content.push(line);
        self.cursor_loc_y += 1;
        self.calc_content_width();
    }

    pub fn process_key_event(
        &mut self,
        KeyEvent { code, modifiers: _ }: KeyEvent,
    ) {
        match code {
            KeyCode::Char(c) => {
                self.add_char(c);
            }
            KeyCode::Enter => {
                if let Some(line) = self.content.get_mut(self.cursor_loc_y) {
                    let new_line = line.split_off(self.cursor_loc_x);
                    self.cursor_loc_y += 1;
                    self.cursor_loc_x = 0;
                    self.content.insert(self.cursor_loc_y, new_line);
                    self.calc_content_width();
                }
            }
            KeyCode::Left => {
                if self.cursor_loc_x > 0 {
                    self.cursor_loc_x -= 1;
                }
            }
            KeyCode::Right => {
                if let Some(line) = self.content.get(self.cursor_loc_y) {
                    if self.cursor_loc_x < line.len() {
                        self.cursor_loc_x += 1;
                    }
                }
            }
            KeyCode::Up => {
                if self.cursor_loc_y > 0 {
                    self.cursor_loc_y -= 1;
                    if let Some(line) = self.content.get(self.cursor_loc_y) {
                        if self.cursor_loc_x > line.len() {
                            self.cursor_loc_x = line.len();
                        }
                    }
                }
            }
            KeyCode::Down => {
                if self.cursor_loc_y < self.content.len() - 1 {
                    self.cursor_loc_y += 1;
                    if let Some(line) = self.content.get(self.cursor_loc_y) {
                        if self.cursor_loc_x > line.len() {
                            self.cursor_loc_x = line.len();
                        }
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(line) = self.content.get_mut(self.cursor_loc_y) {
                    if self.cursor_loc_x > 0 && line.len() > 0 {
                        self.cursor_loc_x -= 1;
                        line.remove(self.cursor_loc_x);
                    }
                    self.calc_content_width();
                }
            }
            KeyCode::Delete => {
                if let Some(line) = self.content.get_mut(self.cursor_loc_y) {
                    if self.cursor_loc_x < line.len() {
                        line.remove(self.cursor_loc_x);
                    }
                    self.calc_content_width();
                }
            }
            _ => (),
        }
    }

    pub fn set_cursor_loc(&mut self, cursor_x: usize, cursor_y: usize) {
        self.cursor_loc_x = cursor_x;
        self.cursor_loc_y = cursor_y;
    }

    pub fn get_cursor_location(&self) -> Vec2 {
        vec2(self.cursor_loc_x as f32, self.cursor_loc_y as f32)
    }

    pub fn height(&self) -> usize {
        self.content.len()
    }

    pub fn width(&self) -> usize {
        self.content_width
    }
}

impl From<String> for AreaBuffer {
    fn from(s: String) -> Self {
        let mut content = vec![];
        let mut cursor_loc_x = 0;
        let mut cursor_loc_y = 0;
        let mut content_width = 0;
        for line in s.lines() {
            cursor_loc_x = 0;
            let mut row = vec![];
            for ch in line.chars() {
                row.push(ch);
                cursor_loc_x += 1;
                if let Some(width) = ch.width() {
                    for _i in 1..width {
                        row.push('\0');
                        cursor_loc_x += 1;
                    }
                }
            }
            if content_width < row.len() {
                content_width = row.len();
            }
            content.push(row);
            cursor_loc_y += 1;
        }
        if cursor_loc_y > 0 {
            cursor_loc_y = cursor_loc_y - 1;
        }

        AreaBuffer {
            content,
            content_width,
            cursor_loc_x,
            cursor_loc_y,
        }
    }
}

impl ToString for AreaBuffer {
    fn to_string(&self) -> String {
        let mut lines = vec![];
        for row in self.content.iter() {
            let row_contents: Vec<String> = row
                .iter()
                .filter(|ch| **ch != '\0')
                .map(ToString::to_string)
                .collect();
            let line = row_contents.join("").trim_end().to_string();
            lines.push(line);
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn add_char1() {
        let s = "The quick brown fox ".to_string();
        let mut area_buffer = AreaBuffer::from(s);
        assert_eq!(0, area_buffer.cursor_loc_y);
        assert_eq!(20, area_buffer.cursor_loc_x);
        area_buffer.add_char('j');
        assert_eq!("The quick brown fox j", area_buffer.to_string());
    }

    #[test]
    fn add_char2() {
        let s = "The quick brown fox ".to_string();
        let mut area_buffer = AreaBuffer::from(s);
        assert_eq!(0, area_buffer.cursor_loc_y);
        assert_eq!(20, area_buffer.cursor_loc_x);
        area_buffer.add_char('j');
        assert_eq!(21, area_buffer.cursor_loc_x);
        area_buffer.add_char('u');
        assert_eq!(22, area_buffer.cursor_loc_x);
        assert_eq!("The quick brown fox ju", area_buffer.to_string());
    }

    #[test]
    fn add_enter() {
        let s = "The quick brown fox ".to_string();
        let mut area_buffer = AreaBuffer::from(s);
        assert_eq!(0, area_buffer.cursor_loc_y);
        assert_eq!(20, area_buffer.cursor_loc_x);
        area_buffer.process_key_event(KeyCode::Enter.into());
        assert_eq!(1, area_buffer.cursor_loc_y);
        assert_eq!(0, area_buffer.cursor_loc_x);
        area_buffer.process_key_event(KeyCode::Char('j').into());
        assert_eq!(1, area_buffer.cursor_loc_y);
        assert_eq!(1, area_buffer.cursor_loc_x);
    }
}
