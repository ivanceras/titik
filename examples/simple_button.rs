pub use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent,
    },
    execute,
    queue,
    style,
    style::{
        Attribute,
        Attributes,
        Color,
        ContentStyle,
    },
    terminal::{
        self,
        ClearType,
    },
    Command,
    Result,
};
use std::{
    fmt::Display,
    io::{
        self,
        Write,
    },
};
use stretch::{
    geometry::Size,
    number::Number,
    style::{
        Dimension,
        FlexDirection,
        Style,
        *,
    },
};
use tuix::{
    compute_layout,
    Box,
    Buffer,
    Button,
    Cell,
    Control,
};

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;
        let (width, height) = buffer_size().unwrap();

        let mut btn = Button::new(format!("{}x{}", width, height));
        btn.set_style(Style {
            size: Size {
                width: Dimension::Points(40.0),
                height: Dimension::Points(3.0),
            },
            ..Default::default()
        });

        let mut btn2 = Button::new("btn2");
        btn2.set_style(Style {
            size: Size {
                width: Dimension::Points(20.0),
                height: Dimension::Points(3.0),
            },
            ..Default::default()
        });

        let mut root_node = Box::default();
        root_node.set_style(Style {
            max_size: Size {
                width: Dimension::Points(width as f32),
                height: Dimension::Points(height as f32),
            },
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            ..Default::default()
        });
        let mut ctrl = Control::Box(root_node);
        ctrl.add_child(btn);
        ctrl.add_child(btn2);
        let layout_tree = compute_layout(
            &mut ctrl,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );
        let mut buf = Buffer::new(width as usize, height as usize);
        ctrl.draw(&mut buf, layout_tree);
        write!(w, "{}", buf);
        w.flush()?;

        match read_char()? {
            '4' => test::run(w)?,
            'q' => break,
            _ => {}
        };
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> Result<char> {
    loop {
        let ev = event::read();
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = ev
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}

mod test {

    use crossterm::{
        cursor::position,
        event::{
            read,
            EnableMouseCapture,
            Event,
            KeyCode,
        },
        execute,
        Result,
    };
    use std::io::Write;

    macro_rules! run_tests {
    (
        $dst:expr,
        $(
            $testfn:ident
        ),*
        $(,)?
    ) => {
        use crossterm::{queue, style, terminal, cursor};
        $(
            queue!(
                $dst,
                style::ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(1, 1),
                cursor::Show,
                cursor::EnableBlinking
            )?;

            $testfn($dst)?;

            match $crate::read_char() {
                Ok('q') => return Ok(()),
                Err(e) => return Err(e),
                _ => { },
            };
        )*
    }
}

    fn test_event<W>(w: &mut W) -> Result<()>
    where
        W: Write,
    {
        execute!(w, EnableMouseCapture)?;

        loop {
            // Blocking read
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }

        Ok(())
    }

    pub fn run<W>(w: &mut W) -> Result<()>
    where
        W: Write,
    {
        run_tests!(w, test_event);
        Ok(())
    }
}
