use crossterm::{
    queue,
    style::{
        Attribute,
        Color,
        ContentStyle,
        ResetColor,
        SetAttributes,
        SetBackgroundColor,
        SetForegroundColor,
    },
};
use std::fmt;
use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
pub struct Cell {
    pub symbol: String,
    pub style: ContentStyle,
}

pub struct Buffer {
    pub cells: Vec<Vec<Cell>>,
}

impl Cell {
    pub fn new<S>(symbol: S) -> Self
    where
        S: ToString,
    {
        Cell {
            symbol: symbol.to_string(),
            style: ContentStyle::default(),
        }
    }

    pub fn unicode_width(&self) -> usize {
        UnicodeWidthStr::width(&*self.symbol)
    }

    pub fn empty() -> Self {
        Cell {
            symbol: " ".to_string(),
            style: ContentStyle::default(),
        }
    }

    pub fn attributes(&mut self, attributes: Vec<Attribute>) {
        for attr in attributes {
            self.style.attributes.set(attr);
        }
    }

    pub fn color(&mut self, color: Color) {
        self.style.foreground_color = Some(color);
    }

    pub fn background(&mut self, color: Color) {
        self.style.background_color = Some(color);
    }
}

impl Buffer {
    /// create a buffer with size
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Cell::empty()).collect())
            .collect();
        Buffer { cells }
    }

    pub fn set_symbol<S: ToString>(&mut self, x: usize, y: usize, symbol: S) {
        self.set_cell(x, y, Cell::new(symbol));
    }

    pub fn set_cell(&mut self, x: usize, y: usize, new_cell: Cell) {
        if let Some(line) = self.cells.get_mut(y) {
            if let Some(cell) = line.get_mut(x) {
                let unicode_width = new_cell.unicode_width();
                *cell = new_cell;
                if unicode_width > 1 {
                    for i in 1..unicode_width {
                        self.set_symbol(x, y + i, '\0');
                    }
                }
            }
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(bg) = self.style.background_color {
            queue!(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
        }
        if let Some(fg) = self.style.foreground_color {
            queue!(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
        }
        if !self.style.attributes.is_empty() {
            queue!(f, SetAttributes(self.style.attributes))
                .map_err(|_| fmt::Error)?;
        }
        self.symbol.fmt(f)?;
        queue!(f, ResetColor).map_err(|_| fmt::Error)?;
        Ok(())
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.iter() {
            for cell in line.iter() {
                cell.fmt(f)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::symbol;
    use std::fmt::Write;

    #[test]
    fn cell1() {
        let mut w = String::new();
        let mut cell = Cell::new("H".to_string());
        write!(w, "{}", cell);
        println!("{}", w);
        assert_eq!(w, "H\u{1b}[0m");
        assert_eq!(cell.unicode_width(), 1);
    }

    #[test]
    fn cell_width() {
        let mut w = String::new();
        let mut cell = Cell::new(symbol::RADIO_UNCHECKED);
        write!(w, "{}", cell);
        println!("{}", w);
        assert_eq!(cell.unicode_width(), 2);
    }

    #[test]
    fn cell2() {
        let mut w = String::new();
        let mut cell = Cell::new("H".to_string());
        cell.attributes(vec![
            Attribute::Bold,
            Attribute::Italic,
            Attribute::CrossedOut,
        ]);
        cell.color(Color::Red);
        cell.background(Color::Yellow);
        write!(w, "{}", cell);
        println!("{}", w);
        assert_eq!(
            w,
            "\u{1b}[48;5;11m\u{1b}[38;5;9m\u{1b}[1m\u{1b}[3m\u{1b}[9mH\u{1b}[0m"
        );
    }
}
