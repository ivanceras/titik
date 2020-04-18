use crossterm::{
    cursor, queue,
    style::{
        Attribute, Attributes, Color, Print, ResetColor, SetAttributes,
        SetBackgroundColor, SetForegroundColor,
    },
};
use std::{fmt, io::Write};
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Cell {
    /// the character symbol
    pub symbol: String,
    /// The foreground color.
    pub foreground_color: Option<Color>,
    /// The background color.
    pub background_color: Option<Color>,
    /// List of attributes.
    pub attributes: Attributes,
}

#[derive(PartialEq, Debug)]
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
            ..Default::default()
        }
    }

    pub fn unicode_width(&self) -> usize {
        UnicodeWidthStr::width(&*self.symbol)
    }

    pub fn empty() -> Self {
        Cell {
            symbol: " ".to_string(),
            ..Default::default()
        }
    }

    pub fn bold(&mut self) {
        self.attributes.set(Attribute::Bold);
    }

    pub fn is_blank(&self) -> bool {
        self.symbol == " "
    }

    pub fn is_filler(&self) -> bool {
        self.symbol == "\0"
    }

    pub fn attributes(&mut self, attributes: Vec<Attribute>) {
        for attr in attributes {
            self.attributes.set(attr);
        }
    }

    pub fn color(&mut self, color: Color) {
        self.foreground_color = Some(color);
    }

    pub fn background(&mut self, color: Color) {
        self.background_color = Some(color);
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

    pub fn reset(&mut self) {
        self.cells.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|cell| *cell = Cell::empty())
        })
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
                        //TODO: this needs to be x + i ;
                        self.set_symbol(x + i, y, '\0');
                    }
                }
            }
        }
    }

    /// get the diff of 2 buffers
    pub fn diff<'a>(&self, new: &'a Self) -> Vec<(usize, usize, &'a Cell)> {
        let mut patches = vec![];
        for (j, new_line) in new.cells.iter().enumerate() {
            for (i, new_cell) in new_line.iter().enumerate() {
                let old_cell =
                    self.cells.get(j).map(|line| line.get(i)).flatten();
                if old_cell != Some(new_cell) {
                    patches.push((i, j, new_cell))
                }
            }
        }
        patches
    }

    pub fn render<W: Write>(&self, w: &mut W) -> crossterm::Result<()> {
        crossterm::queue!(w, cursor::Hide)?;
        for (j, line) in self.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                crossterm::queue!(w, cursor::MoveTo(i as u16, j as u16))?;
                // fillter is \0 null character, filler is not printable
                if !cell.is_filler() {
                    crossterm::queue!(w, Print(cell))?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(bg) = self.background_color {
            queue!(f, SetBackgroundColor(bg)).map_err(|_| fmt::Error)?;
        }
        if let Some(fg) = self.foreground_color {
            queue!(f, SetForegroundColor(fg)).map_err(|_| fmt::Error)?;
        }
        if !self.attributes.is_empty() {
            queue!(f, SetAttributes(self.attributes))
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
        let cell = Cell::new("H".to_string());
        write!(w, "{}", cell);
        println!("{}", w);
        assert_eq!(w, "H\u{1b}[0m");
        assert_eq!(cell.unicode_width(), 1);
    }

    #[test]
    fn cell_width() {
        let mut w = String::new();
        let cell = Cell::new(symbol::RADIO_UNCHECKED);
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

    #[test]
    fn diff1() {
        let mut buf = Buffer::new(10, 10);
        buf.set_symbol(1, 1, 'H');
        buf.set_symbol(1, 2, 'e');

        let mut buf2 = Buffer::new(10, 10);
        buf2.set_symbol(1, 1, 'A');
        buf2.set_symbol(1, 2, 'B');

        let dif = buf.diff(&buf2);
        for (x, y, cell) in dif.iter() {
            println!("diff: {},{}: {}", x, y, cell);
        }
        assert_eq!((1, 1, &Cell::new('A')), dif[0]);
        assert_eq!((1, 2, &Cell::new('B')), dif[1]);
        buf2.reset();
        assert_eq!(Cell::new(' '), buf2.cells[1][1]);
        assert_eq!(Cell::new(' '), buf2.cells[1][2]);
    }
}
