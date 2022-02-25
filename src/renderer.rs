//! Provides the core functionality of rendering to the terminal
//! This has the event loop which calculates and process the events to the target widget

use crate::Event;
use crate::{command, find_node, Buffer, Widget};

use crate::crossterm::{
    event::{self, KeyCode, KeyModifiers},
    terminal::{self},
    Result,
};
use expanse::{geometry::Size, number::Number};
use std::io::Stdout;
use std::io::Write;

/// A Dispatch trait which the implementing APP will update
/// its own state based on the supplied msg.
pub trait Dispatch<MSG> {
    /// dispatch the msg and passed the root node for the implementing
    /// app to access it and change the state of the UI.
    fn dispatch(&self, msg: MSG, root_node: &mut dyn Widget<MSG>);
}

/// This provides the render loop of the terminal UI
pub struct Renderer<'a, MSG> {
    write: Stdout,
    program: Option<&'a dyn Dispatch<MSG>>,
    root_node: &'a mut dyn Widget<MSG>,
    terminal_size: (u16, u16),
    focused_widget_idx: Option<usize>,
}

impl<'a, MSG> Renderer<'a, MSG> {
    /// create a new renderer with the supplied root_node
    pub fn new(
        write: Stdout,
        program: Option<&'a dyn Dispatch<MSG>>,
        root_node: &'a mut dyn Widget<MSG>,
    ) -> Self {
        let (width, height) =
            terminal::size().expect("must get the terminal size");

        root_node.compute_node_layout(Size {
            width: Number::Defined(width as f32),
            height: Number::Defined(height as f32),
        });
        Renderer {
            write,
            program,
            root_node,
            terminal_size: (width, height),
            focused_widget_idx: None,
        }
    }

    fn recompute_layout(&mut self) {
        let (width, height) = self.terminal_size;
        self.root_node.compute_node_layout(Size {
            width: Number::Defined(width as f32),
            height: Number::Defined(height as f32),
        });
    }

    fn dispatch_msg(&mut self, msgs: Vec<MSG>) {
        if let Some(program) = self.program {
            for msg in msgs {
                program.dispatch(msg, self.root_node);
            }
        }
        self.recompute_layout();
    }

    #[cfg(feature = "crossterm_new")]
    fn is_mouse_move(c_event: crate::crossterm::event::Event) -> bool {
        match c_event {
            crate::crossterm::event::Event::Mouse(me) => match me.kind {
                crate::crossterm::event::MouseEventKind::Moved => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[cfg(feature = "crossterm")]
    fn is_mouse_move(_c_event: crate::crossterm::event::Event) -> bool {
        false
    }

    /// run the event loop of the renderer
    pub fn run(&mut self) -> Result<()> {
        command::init(&mut self.write)?;
        command::reset_top(&mut self.write)?;
        let (width, height) = self.terminal_size;

        loop {
            let mut buf = Buffer::new(width as usize, height as usize);

            buf.reset();
            let cmds = self.root_node.draw_widget(&mut buf)?;
            buf.render(&mut self.write)?;

            cmds.iter().for_each(|cmd| {
                cmd.execute(&mut self.write).expect("must execute")
            });
            self.write.flush()?;

            if let Ok(c_event) = event::read() {
                if !Self::is_mouse_move(c_event) {
                    let event = Event::from_crossterm(c_event);
                    let should_break = self.process_event(&event);
                    if should_break {
                        break;
                    }
                    // any other activities, such as mouse scroll is
                    // sent the widget underneath the location, regardless
                    // if it focused or not.

                    if let Some((x, y)) = event.extract_location() {
                        let hits = self
                            .root_node
                            .node_hit_at(x as f32, y as f32, &mut 0);
                        for hit in hits.iter().rev() {
                            let mut hit_widget: Option<&mut dyn Widget<MSG>> =
                                find_node::find_widget_mut(
                                    self.root_node,
                                    *hit,
                                );

                            if let Some(hit_widget) = &mut hit_widget {
                                let msgs =
                                    hit_widget.process_event(event.clone());
                                self.dispatch_msg(msgs);
                            }
                        }
                    }
                }
            }
        }
        command::finalize(&mut self.write)?;
        Ok(())
    }

    /// process the event
    fn process_event(&mut self, event: &Event) -> bool {
        match event {
            Event::Key(key_event) => {
                // To quite, press any of the following:
                //  - CTRL-c
                //  - CTRL-q
                //  - CTRL-d
                //  - CTRL-z
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    match key_event.code {
                        KeyCode::Char(c) => match c {
                            'c' | 'q' | 'd' | 'z' => {
                                return true;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                } else {
                    // send the keypresses to the focused widget
                    if let Some(idx) = self.focused_widget_idx.as_ref() {
                        let active_widget: Option<&mut dyn Widget<MSG>> =
                            find_node::find_widget_mut(self.root_node, *idx);
                        if let Some(focused_widget) = active_widget {
                            let msgs =
                                focused_widget.process_event(event.clone());
                            self.dispatch_msg(msgs);
                        }
                    }
                }
            }
            // mouse clicks sets the focused the widget underneath
            Event::Mouse(_me) => {
                if event.is_mouse_click() {
                    let (x, y) = event
                        .extract_location()
                        .expect("must have a mouse location");
                    self.focused_widget_idx = self
                        .root_node
                        .node_hit_at(x as f32, y as f32, &mut 0)
                        .pop();

                    if let Some(idx) = self.focused_widget_idx.as_ref() {
                        self.root_node.set_focused_node(*idx);
                    }
                }
            }
            Event::Resize(width, height) => {
                self.terminal_size = (*width, *height);
                self.recompute_layout();
            }
            _ => (),
        }
        false
    }
}
