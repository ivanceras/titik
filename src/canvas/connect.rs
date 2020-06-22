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

lazy_static! {
    /// A lookup for getting the connect property from a character
    static ref CHAR_CONNECT_PROPERTIES: BTreeMap<char, Connect> =
        BTreeMap::from_iter(Connect::all_connect_property());

    /// A lookup for getting the character from a connect property
    pub(crate) static ref CONNECT_CHAR_PROPERTIES: BTreeMap<Connect, char> =
        BTreeMap::from_iter(Connect::all_connect_property()
            .into_iter()
            .map(|(ch, connect)| (connect, ch)));
}

///```ignore
///          C
///     A┌───┬───┐E
///      │   │   │
///      │   │   │
///      │   │   │
///     K├───M───┤O
///      │   │   │
///      │   │   │
///      │   │   │
///     U└───┴───┘Y
///          W
///```

pub(crate) enum Cell {
    A,
    C,
    K,
    M,
    O,
    U,
    W,
    Y,
}

pub(crate) enum Fragment {
    Line(Line),
    Arc(Arc),
}

pub struct Line {
    start: Cell,
    end: Cell,
    is_thick: bool,
}

pub(crate) fn thick_line(start: Cell, end: Cell) -> Fragment {
    let line = Line {
        start,
        end,
        is_thick: true,
    };
    Fragment::Line(line)
}

pub(crate) fn line(start: Cell, end: Cell) -> Fragment {
    let line = Line {
        start,
        end,
        is_thick: false,
    };
    Fragment::Line(line)
}

pub(crate) fn arc(start: Cell, end: Cell) -> Fragment {
    let arc = Arc { start, end };
    Fragment::Arc(arc)
}

pub struct Arc {
    start: Cell,
    end: Cell,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Connect {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Connect {
    pub(crate) fn from_char(ch: char) -> Self {
        let mut top = false;
        let mut bottom = false;
        let mut left = false;
        let mut right = false;

        match ch {
            '╴' => {
                left = true;
            }
            '╵' => {
                top = true;
            }
            '╶' => {
                right = true;
            }
            '╷' => {
                bottom = true;
            }
            '─' => {
                left = true;
                right = true;
            }
            '│' => {
                top = true;
                bottom = true;
            }
            '┌' => {
                right = true;
                bottom = true;
            }
            '┐' => {
                left = true;
                bottom = true;
            }
            '└' => {
                top = true;
                right = true;
            }
            '┘' => {
                top = true;
                left = true;
            }
            '┬' => {
                left = true;
                right = true;
                bottom = true;
            }
            '┴' => {
                top = true;
                left = true;
                right = true;
            }
            '├' => {
                top = true;
                bottom = true;
                right = true;
            }
            '┤' => {
                top = true;
                bottom = true;
                left = true;
            }
            '┼' => {
                top = true;
                bottom = true;
                left = true;
                right = true;
            }
            '╭' => {
                right = true;
                bottom = true;
            }
            '╮' => {
                left = true;
                bottom = true;
            }
            '╰' => {
                top = true;
                right = true;
            }
            '╯' => {
                top = true;
                left = true;
            }
            _ => (),
        }

        Connect {
            top,
            bottom,
            left,
            right,
        }
    }

    fn all_char() -> [char; 19] {
        [
            '╴', '╵', '╶', '╷', '─', '│', '┌', '┐', '└', '┘', '┬', '┴', '├',
            '┤', '┼', '╭', '╮', '╰', '╯',
        ]
    }

    /// get the intersection of this connect property
    pub(crate) fn intersect(&self, other: &Self) -> Self {
        let mut this = self.clone();
        this.top |= other.top;
        this.bottom |= other.bottom;
        this.left |= other.left;
        this.right |= other.right;
        this
    }

    /// all property
    fn all_connect_property() -> Box<dyn Iterator<Item = (char, Self)>> {
        Box::new(
            Self::all_char()
                .to_vec()
                .into_iter()
                .map(|ch| (ch, Self::from_char(ch))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let cross = Connect::from_char('┼');
        assert_eq!(
            cross,
            Connect {
                top: true,
                bottom: true,
                left: true,
                right: true
            }
        )
    }

    #[test]
    fn test_assembled_cross_and_corner() {
        let cross = Connect::from_char('┼');
        let bottom_left = Connect::from_char('└');
        let top_right = Connect::from_char('┐');
        let intersection = bottom_left.intersect(&top_right);
        assert_eq!(cross, intersection);
    }

    #[test]
    fn test_assembled_cross_and_line() {
        let cross = Connect::from_char('┼');
        let vertical = Connect::from_char('│');
        let horizontal = Connect::from_char('─');
        let intersection = vertical.intersect(&horizontal);
        assert_eq!(cross, intersection);
    }

    #[test]
    fn test_assembled_halflings() {
        let bottom_left = Connect::from_char('└');
        let top = Connect::from_char('╵');
        let right = Connect::from_char('╶');
        let mut intersection = bottom_left.intersect(&top);
        intersection = intersection.intersect(&right);
        assert_eq!(bottom_left, intersection);
    }
}
