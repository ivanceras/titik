use crate::Value;
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone, PartialEq)]
pub struct InputEvent {
    value: Value,
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

impl Event {
    pub fn from_crossterm(c_event: crossterm::event::Event) -> Self {
        match c_event {
            crossterm::event::Event::Key(ke) => todo!(),
            crossterm::event::Event::Mouse(me) => todo!(),
            crossterm::event::Event::Resize(width, height) => {
                Event::Resize(width, height)
            }
        }
    }
}
