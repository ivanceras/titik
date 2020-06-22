use crate::{
    symbol,
    symbol::{
        bar,
        line,
        rounded,
        thick_line,
    },
};
use connect::{
    Connect,
    CONNECT_CHAR_PROPERTIES,
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
pub(crate) struct Canvas {
    cells: HashMap<(usize, usize), Vec<char>>,
}

pub(crate) struct Border {
    pub(crate) use_thick_border: bool,

    pub(crate) has_top: bool,
    pub(crate) has_bottom: bool,
    pub(crate) has_left: bool,
    pub(crate) has_right: bool,

    pub(crate) is_top_left_rounded: bool,
    pub(crate) is_top_right_rounded: bool,
    pub(crate) is_bottom_left_rounded: bool,
    pub(crate) is_bottom_right_rounded: bool,
}

impl Border {
    fn horizontal_symbol(&self) -> char {
        if self.use_thick_border {
            thick_line::HORIZONTAL
        } else {
            line::HORIZONTAL
        }
    }

    fn top_symbol(&self) -> char {
        if self.has_top {
            self.horizontal_symbol()
        } else {
            symbol::EMPTY
        }
    }

    fn bottom_symbol(&self) -> char {
        if self.has_bottom {
            self.horizontal_symbol()
        } else {
            symbol::EMPTY
        }
    }

    fn vertical_symbol(&self) -> char {
        if self.use_thick_border {
            thick_line::VERTICAL
        } else {
            line::VERTICAL
        }
    }

    fn left_symbol(&self) -> char {
        if self.has_left {
            self.vertical_symbol()
        } else {
            symbol::EMPTY
        }
    }

    fn right_symbol(&self) -> char {
        if self.has_right {
            self.vertical_symbol()
        } else {
            symbol::EMPTY
        }
    }

    fn top_left_symbol(&self) -> char {
        if self.use_thick_border {
            thick_line::TOP_LEFT
        } else if self.is_top_left_rounded {
            rounded::TOP_LEFT
        } else {
            line::TOP_LEFT
        }
    }

    fn top_right_symbol(&self) -> char {
        if self.use_thick_border {
            thick_line::TOP_RIGHT
        } else if self.is_top_right_rounded {
            rounded::TOP_RIGHT
        } else {
            line::TOP_RIGHT
        }
    }

    fn bottom_left_symbol(&self) -> char {
        if self.use_thick_border {
            thick_line::BOTTOM_LEFT
        } else if self.is_bottom_left_rounded {
            rounded::BOTTOM_LEFT
        } else {
            line::BOTTOM_LEFT
        }
    }

    fn bottom_right_symbol(&self) -> char {
        if self.use_thick_border {
            thick_line::BOTTOM_RIGHT
        } else if self.is_bottom_right_rounded {
            rounded::BOTTOM_RIGHT
        } else {
            line::BOTTOM_RIGHT
        }
    }

    fn get_symbols(&self) -> (char, char, char, char, char, char, char, char) {
        (
            self.top_left_symbol(),
            self.top_symbol(),
            self.top_right_symbol(),
            self.right_symbol(),
            self.bottom_right_symbol(),
            self.bottom_symbol(),
            self.bottom_left_symbol(),
            self.left_symbol(),
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
    pub(crate) fn new() -> Self {
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

    pub(crate) fn draw_horizontal_line(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        horizontal_symbol: char,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;
        assert_eq!(
            y1, y2,
            "horizontal line must have the same start.y and end.y"
        );
        let width = x2 - x1;
        for i in 0..width {
            // horizontal line at the top
            self.add_char(x1 + i, y1, horizontal_symbol);
        }
    }

    pub(crate) fn draw_vertical_line(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        vertical_symbol: char,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;
        assert_eq!(
            x1, x2,
            "vertical line must have the same start.x and end.x"
        );
        let height = y2 - y1;
        for j in 0..height {
            // vertical line at the left side
            self.add_char(x1, y1 + j, vertical_symbol);
        }
    }

    pub(crate) fn draw_rect(
        &mut self,
        start: (usize, usize),
        end: (usize, usize),
        border: Border,
    ) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        let left = x1;
        let top = y1;
        let right = x2;
        let bottom = y2;

        let (
            top_left_symbol,
            top_symbol,
            top_right_symbol,
            right_symbol,
            bottom_right_symbol,
            bottom_symbol,
            bottom_left_symbol,
            left_symbol,
        ) = border.get_symbols();

        // draw the top border;
        self.draw_horizontal_line((left + 1, top), (right, top), top_symbol);

        // draw the bottom border;
        self.draw_horizontal_line(
            (left + 1, bottom),
            (right, bottom),
            bottom_symbol,
        );

        // left border
        self.draw_vertical_line((left, top + 1), (left, bottom), left_symbol);

        // right border
        self.draw_vertical_line(
            (right, top + 1),
            (right, bottom),
            right_symbol,
        );

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
            let mut base_connect = Connect::from_char(chars[0]);
            for ch in chars.iter().skip(1) {
                let ch_connect = Connect::from_char(*ch);
                base_connect = base_connect.intersect(&ch_connect);
            }
            CONNECT_CHAR_PROPERTIES.get(&base_connect).map(|c| *c)
        }
    }

    /// resolve each group of characters in the cells and return an iterator
    pub(crate) fn get_cells<'a>(
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

        canvas.draw_rect((0, 0), (2, 2), Border::default());
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
