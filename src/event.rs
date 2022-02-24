use crate::Value;

#[cfg(feature = "crossterm_new")]
use crossterm_new as crossterm;

use crossterm::event::MouseEventKind;
pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

#[derive(Debug, Clone, PartialEq)]
pub struct InputEvent {
    pub value: Value,
}

impl From<Value> for InputEvent {
    fn from(value: Value) -> Self {
        InputEvent { value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    InputEvent(InputEvent),
    Resize(u16, u16),
}

impl From<InputEvent> for Event {
    fn from(ie: InputEvent) -> Self {
        Event::InputEvent(ie)
    }
}

impl From<KeyEvent> for Event {
    fn from(ke: KeyEvent) -> Self {
        Self::Key(ke)
    }
}

impl Event {
    pub fn from_crossterm(c_event: crossterm::event::Event) -> Self {
        match c_event {
            crossterm::event::Event::Key(ke) => Event::Key(ke),
            crossterm::event::Event::Mouse(me) => Event::Mouse(me),
            crossterm::event::Event::Resize(width, height) => {
                Event::Resize(width, height)
            }
        }
    }

    /*
    pub fn is_mouse_click(&self) -> bool {
        match self {
            Event::Mouse(me) => match me {
                MouseEvent::Down(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_mouse_drag(&self) -> bool {
        match self {
            Event::Mouse(me) => match me {
                MouseEvent::Drag(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_scrollup(&self) -> bool {
        match self {
            Event::Mouse(me) => match me {
                MouseEvent::ScrollUp(..) => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_scrolldown(&self) -> bool {
        match self {
            Event::Mouse(me) => match me {
                MouseEvent::ScrollDown(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn modifiers(&self) -> Option<&KeyModifiers> {
        match self {
            Event::Mouse(me) => match me {
                MouseEvent::Down(.., ref modifier) => Some(modifier),
                MouseEvent::Up(.., ref modifier) => Some(modifier),
                MouseEvent::Drag(.., ref modifier) => Some(modifier),
                MouseEvent::ScrollDown(.., ref modifier) => Some(modifier),
                MouseEvent::ScrollUp(.., ref modifier) => Some(modifier),
            },
            Event::Key(ke) => Some(&ke.modifiers),
            _ => None,
        }
    }

    /// extract the x and y location of a mouse event
    pub fn extract_location(&self) -> Option<(u16, u16)> {
        match self {
            Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => {
                Some((*x, *y))
            }
            Event::Mouse(MouseEvent::Up(_btn, x, y, _modifier)) => {
                Some((*x, *y))
            }
            Event::Mouse(MouseEvent::Drag(_btn, x, y, _modifier)) => {
                Some((*x, *y))
            }
            Event::Mouse(MouseEvent::ScrollDown(x, y, _modifier)) => {
                Some((*x, *y))
            }
            Event::Mouse(MouseEvent::ScrollUp(x, y, _modifier)) => {
                Some((*x, *y))
            }
            Event::Key(_) => None,
            Event::Resize(_, _) => None,
            Event::InputEvent(_) => None,
        }
    }
    */

    pub fn is_mouse_click(&self) -> bool {
        match self {
            Event::Mouse(me) => match me.kind {
                MouseEventKind::Down(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_mouse_drag(&self) -> bool {
        match self {
            Event::Mouse(me) => match me.kind {
                MouseEventKind::Drag(..) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_scrollup(&self) -> bool {
        match self {
            Event::Mouse(me) => match me.kind {
                MouseEventKind::ScrollUp => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_scrolldown(&self) -> bool {
        match self {
            Event::Mouse(me) => match me.kind {
                MouseEventKind::ScrollDown => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn extract_location(&self) -> Option<(u16, u16)> {
        match self {
            Event::Mouse(me) => Some((me.column, me.row)),
            _ => None,
        }
    }

    pub fn modifiers(&self) -> Option<&KeyModifiers> {
        match self {
            Event::Mouse(me) => Some(&me.modifiers),
            Event::Key(ke) => Some(&ke.modifiers),
            _ => None,
        }
    }
}
