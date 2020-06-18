use crate::symbol::{
    bar,
    line,
    rounded,
    thick_line,
};
use lazy_static::lazy_static;
use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    iter::FromIterator,
};

mod connect;

/// canvas which only draws rectangluar shapes intended to be used
/// mainly in drawing borders of a widget
/// each location on this canvas can contain multiple character.
///
/// Upon flattening, each cell should resolve to only 1 char.
/// if all the chars one cells can be merge and resolve to a one
/// character then that char will be used, otherwise, the last inserted
/// char will be used
pub struct Canvas {
    cells: HashMap<(usize, usize), Vec<char>>,
}

pub struct Border {
    use_thick_border: bool,

    has_top: bool,
    has_bottom: bool,
    has_left: bool,
    has_right: bool,

    is_top_left_rounded: bool,
    is_top_right_rounded: bool,
    is_bottom_left_rounded: bool,
    is_bottom_right_rounded: bool,
}

impl Border {
    fn get_symbols(&self) -> (char, char, char, char, char, char) {
        let mut top_left_symbol = if self.is_top_left_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        };

        let mut top_right_symbol = if self.is_top_right_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        };
        let mut bottom_left_symbol = if self.is_bottom_left_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        };
        let mut bottom_right_symbol = if self.is_bottom_right_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        };

        let mut horizontal_symbol = line::HORIZONTAL;
        let mut vertical_symbol = line::VERTICAL;

        // Note: the rounded border is override with square thick line since there is no thick
        // rounded corner
        if self.use_thick_border {
            top_left_symbol = thick_line::TOP_LEFT;
            top_right_symbol = thick_line::TOP_RIGHT;
            bottom_left_symbol = thick_line::BOTTOM_LEFT;
            bottom_right_symbol = thick_line::BOTTOM_RIGHT;
            horizontal_symbol = thick_line::HORIZONTAL;
            vertical_symbol = thick_line::VERTICAL;
        }

        (
            top_left_symbol,
            top_right_symbol,
            bottom_left_symbol,
            bottom_right_symbol,
            horizontal_symbol,
            vertical_symbol,
        )
    }
}

impl Default for Border {
    fn default() -> Self {
        Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        }
    }
}

impl Canvas {
    fn new() -> Self {
        Canvas {
            cells: HashMap::new(),
        }
    }

    fn add_char(&mut self, i: usize, j: usize, ch: char) {
        if let Some(existing) = self.cells.get_mut(&(i, j)) {
            existing.push(ch);
        } else {
            self.cells.insert((i, j), vec![ch]);
        }
    }

    fn draw_rect(
        &mut self,
        start: (f32, f32),
        end: (f32, f32),
        border: Border,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        let (x1, y1) = (x1.round() as usize, y1.round() as usize);
        let (x2, y2) = (x2.round() as usize, y2.round() as usize);

        let width = x2 - x1;
        let height = y2 - y1;
        let left = x1;
        let top = y1;
        let right = x2;
        let bottom = y2;

        let (
            top_left_symbol,
            top_right_symbol,
            bottom_left_symbol,
            bottom_right_symbol,
            horizontal_symbol,
            vertical_symbol,
        ) = border.get_symbols();

        // draw the top and bottom border;
        for i in 1..width {
            // horizontal line at the top
            self.add_char(left + i, top, horizontal_symbol);

            // horizontal line at the bottom
            self.add_char(left + i, bottom, horizontal_symbol);
        }

        // draw the left and right border
        for j in 1..height {
            // vertical line at the left side
            self.add_char(left, top + j, vertical_symbol);

            // vertical line at the right side
            self.add_char(right, top + j, vertical_symbol);
        }

        // draw the corners
        self.add_char(left, top, top_left_symbol);
        self.add_char(left, bottom, bottom_left_symbol);
        self.add_char(right, top, top_right_symbol);
        self.add_char(right, bottom, bottom_right_symbol);
    }

    /// resolve the chars in this cell
    ///
    ///  ['└',  '─'] will be resolve as '┴'
    ///  ['┘',  '─'] will be resolve as '┴'
    ///
    ///  ['│',  '─'] will be resolve as '┼'
    ///
    ///  ['┘',  '┌'] will be resolve as '┼'
    ///
    ///  ['└',  '│'] will be resolve as '┤'
    fn resolve(chars: &[char]) -> Option<char> {
        if chars.len() == 1 {
            Some(chars[0])
        } else {
            let len = chars.len();
            println!("multiple chars {}", len);
            chars.get(len - 1).map(|c| *c);
            todo!();
        }
    }

    /// resolve each group of characters in the cells and return an iterator
    pub fn get_cells<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = (usize, usize, char)> + 'a> {
        Box::new(self.cells.iter().flat_map(|((i, j), chars)| {
            Self::resolve(&chars).map(|ch| (*i, *j, ch))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut canvas = Canvas::new();

        canvas.draw_rect((0.0, 0.0), (2.0, 2.0), Border::default());
        let mut chars: Vec<(usize, usize, char)> = canvas.get_cells().collect();
        chars.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        println!("chars: {:?}", chars);
        assert_eq!(
            chars,
            [
                (0, 0, '┌'),
                (0, 1, '│'),
                (0, 2, '└'),
                (1, 0, '─'),
                (1, 2, '─'),
                (2, 0, '┐'),
                (2, 1, '│'),
                (2, 2, '┘')
            ]
        );
    }
}
