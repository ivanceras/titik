use crate::crossterm::{
    self, cursor, queue,
    style::{
        Attribute, Attributes, Color, Print, ResetColor, SetAttributes,
        SetBackgroundColor, SetForegroundColor,
    },
};
use crate::symbol;
use ito_canvas::unicode_canvas::Canvas;
use std::io::Stdout;
use std::{fmt, io::Write};
use unicode_width::UnicodeWidthStr;

/// Cell contains the attributes of the char used in the buffer.
/// This information is needed when rendering each cell to the terminal
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

/// Contains a vec of cells.
/// Buffer contains the information needed to render into the screen
#[derive(PartialEq, Debug)]
pub struct Buffer {
    pub(crate) cells: Vec<Vec<Cell>>,
}

impl Cell {
    /// create a new Cell from character or string
    pub fn new<S>(symbol: S) -> Self
    where
        S: ToString,
    {
        Cell {
            symbol: symbol.to_string(),
            ..Default::default()
        }
    }

    /// returns the unicode width of the cell.
    /// Some characters are wide such as CJK
    pub fn unicode_width(&self) -> usize {
        UnicodeWidthStr::width(&*self.symbol)
    }

    /// creates an empty Cell
    pub fn empty() -> Self {
        Cell {
            symbol: symbol::EMPTY.to_string(),
            ..Default::default()
        }
    }

    /// render this cell as bold
    pub fn bold(&mut self) {
        self.attributes.set(Attribute::Bold);
    }

    /// whether or not this cell is blank
    pub fn is_blank(&self) -> bool {
        self.symbol == symbol::EMPTY.to_string()
    }

    /// whether or not this cell is a filler of a wide character
    /// that appears before it
    pub fn is_filler(&self) -> bool {
        self.symbol == "\0"
    }

    /// return the attributes of this cell
    pub fn attributes(&mut self, attributes: Vec<Attribute>) {
        for attr in attributes {
            self.attributes.set(attr);
        }
    }

    /// set the foreground color of this cell
    pub fn color(&mut self, color: Color) {
        self.foreground_color = Some(color);
    }

    /// set the background color of this cell
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

    /// reset the content of the buffer to empty
    pub fn reset(&mut self) {
        self.cells.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|cell| *cell = Cell::empty())
        })
    }

    /// set the character of this location with symbol
    pub fn set_symbol<S: ToString>(&mut self, x: usize, y: usize, symbol: S) {
        self.set_cell(x, y, Cell::new(symbol));
    }

    /// enumerate the characters in the string and set the cells horizontally incrementing on the x
    /// component
    pub fn write_str<S: ToString>(&mut self, x: usize, y: usize, s: S) {
        for (i, ch) in s.to_string().chars().enumerate() {
            self.set_cell(x + i, y, Cell::new(ch));
        }
    }

    /// write string as bold
    pub fn write_bold_str<S: ToString>(&mut self, x: usize, y: usize, s: S) {
        for (i, ch) in s.to_string().chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.bold();
            self.set_cell(x + i, y, cell);
        }
    }

    /// get the characters from the drawing canvas and
    /// insert them into this buffer
    pub(crate) fn write_canvas(&mut self, canvas: Canvas) {
        canvas
            .get_cells()
            .for_each(|(x, y, ch)| self.set_symbol(x, y, ch))
    }

    /// set the cell at this location
    pub fn set_cell(&mut self, x: usize, y: usize, new_cell: Cell) {
        if let Some(line) = self.cells.get_mut(y) {
            if let Some(cell) = line.get_mut(x) {
                let unicode_width = new_cell.unicode_width();
                *cell = new_cell;
                if unicode_width > 1 {
                    for i in 1..unicode_width {
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

    /// writes to the stdout buffer
    pub fn render(&self, w: &mut Stdout) -> crossterm::Result<()> {
        queue!(w, cursor::Hide)?;
        for (j, line) in self.cells.iter().enumerate() {
            for (i, cell) in line.iter().enumerate() {
                queue!(w, cursor::MoveTo(i as u16, j as u16))?;
                if let Some(bg) = cell.background_color {
                    queue!(w, SetBackgroundColor(bg))?;
                }
                if let Some(fg) = cell.foreground_color {
                    queue!(w, SetForegroundColor(fg))?;
                }
                if !cell.attributes.is_empty() {
                    queue!(w, SetAttributes(cell.attributes))?;
                }
                // fillter is \0 null character, filler is not printable
                if !cell.is_filler() {
                    queue!(w, Print(cell))?;
                }
                queue!(w, ResetColor)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.symbol.fmt(f)?;
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
        write!(w, "{}", cell).unwrap();
        println!("{}", w);
        assert_eq!(w, "H");
        assert_eq!(cell.unicode_width(), 1);
    }

    #[test]
    fn cell_width() {
        let mut w = String::new();
        let cell = Cell::new(symbol::RADIO_UNCHECKED);
        write!(w, "{}", cell).unwrap();
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
        write!(w, "{}", cell).unwrap();
        println!("{}", w);
        assert_eq!(w, "H");
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
