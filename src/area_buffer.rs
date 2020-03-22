use crossterm::event::{
    KeyCode,
    KeyEvent,
};
use unicode_width::UnicodeWidthChar;

/// Area buffer is a 2 dimensional text buffer
#[derive(Default, Debug, PartialEq, Clone)]
pub struct AreaBuffer {
    pub(crate) content: Vec<Vec<char>>,
    cursor_loc_x: usize,
    cursor_loc_y: usize,
}

impl AreaBuffer {
    pub fn new() -> Self {
        AreaBuffer {
            content: vec![],
            cursor_loc_x: 0,
            cursor_loc_y: 0,
        }
    }

    fn add_char(&mut self, c: char) {
        let line = self.content.get_mut(self.cursor_loc_y).expect("must have a line");
            line.insert(self.cursor_loc_x, c);
            self.cursor_loc_x += 1;
    }

    pub fn add_line<S:ToString>(&mut self, s: S){
        let line = s.to_string().chars().collect();
        self.content.push(line);
        self.cursor_loc_y += 1;
    }

    pub fn process_key_event(
        &mut self,
        KeyEvent { code, modifiers }: KeyEvent,
    ) {
        match code {
            KeyCode::Char(c) => {
                self.add_char(c);
            }
            KeyCode::Enter => {
                self.content.push(vec![]);
                self.cursor_loc_y += 1;
                self.cursor_loc_x = 0;
            }
            _ => (),
        }
    }


    pub fn set_cursor_loc_corrected(&mut self, mut x: i32, mut y: i32) {
        if y < 0 {
            y = 0;
        }
        let rows = self.content.len() as i32;
        if y >= rows  {
            y = rows - 1;
        }
        if x < 0 {
            x = 0;
        }
        let cursor_y = y as usize;
        if let Some(line) = self.content.get(cursor_y){
            if x > line.len() as i32{
                x = line.len() as i32;
            }
        }
        let cursor_x = x as usize;
        self.cursor_loc_x = cursor_x;
        self.cursor_loc_y = cursor_y;
    }

    pub fn get_cursor_location(&self) -> (usize, usize) {
        (self.cursor_loc_x , self.cursor_loc_y )
    }
}

impl From<String> for AreaBuffer {
    fn from(s: String) -> Self {
        let mut content = vec![];
        let mut cursor_loc_x = 0;
        let mut cursor_loc_y = 0;
        for line in s.lines() {
            cursor_loc_x = 0;
            let mut row = vec![];
            for ch in line.chars() {
                row.push(ch);
                cursor_loc_x += 1;
                if let Some(width) = ch.width() {
                    for i in 1..width {
                        row.push('\0');
                        cursor_loc_x += 1;
                    }
                }
            }
            content.push(row);
            cursor_loc_y += 1;
        }

        AreaBuffer {
            content,
            cursor_loc_x: cursor_loc_x,
            cursor_loc_y: cursor_loc_y - 1,
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
        let s = "The quick brown fox ";
        let mut area_buffer = AreaBuffer::from(s);
        assert_eq!(0, area_buffer.cursor_loc_y);
        assert_eq!(20, area_buffer.cursor_loc_x);
        area_buffer.add_char('j');
        assert_eq!("The quick brown fox j", area_buffer.to_string());
    }

    #[test]
    fn add_char2() {
        let s = "The quick brown fox ";
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
        let s = "The quick brown fox ";
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