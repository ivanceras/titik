use crossterm::{
    queue,
    style::{
        Attribute,
        Color,
        ContentStyle,
        SetAttributes,
        SetBackgroundColor,
        SetForegroundColor,
    },
};
use std::fmt;

pub struct Cell {
    pub symbol: String,
    pub style: ContentStyle,
}

pub struct Buffer {
    pub cells: Vec<Vec<Cell>>,
}

impl Cell {
    pub fn new(symbol: String) -> Self {
        Cell {
            symbol,
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

        write!(f, "{}", self.symbol)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn cell1() {
        let mut w = String::new();
        let mut cell = Cell::new("H".to_string());
        write!(w, "{}", cell);
        println!("{}", w);
        assert_eq!(w, "H");
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
            "\u{1b}[48;5;11m\u{1b}[38;5;9m\u{1b}[1m\u{1b}[3m\u{1b}[9mH"
        );
    }
}
