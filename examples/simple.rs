use std::io::{self, Write};

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    style::Attribute,
    style::Attributes,
    style::Color,
    terminal::{self, ClearType},
    Command, Result,
};

const MENU: &str = r#"Crossterm interactive test

Controls:

 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step

Available tests:

1. cursor
2. color (foreground, background)
3. attributes (bold, italic, ...)
4. input

Select test to run ('1', '2', ...) or hit 'q' to quit.
"#;

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

        let mut attributes = Attributes::default();
        attributes.set(Attribute::Bold);
        attributes.set(Attribute::Italic);
        attributes.set(Attribute::CrossedOut);

        for line in MENU.split('\n') {
            queue!(
                w,
                style::SetAttributes(attributes),
                style::SetForegroundColor(Color::Red),
                style::SetBackgroundColor(Color::Green),
                style::Print(line),
                cursor::MoveToNextLine(1),
            )?;
        }

        w.flush()?;

        match read_char()? {
            //'1' => test::cursor::run(w)?,
            //'2' => test::color::run(w)?,
            //'3' => test::attribute::run(w)?,
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
        event::{read, EnableMouseCapture, Event, KeyCode},
        execute, Result,
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
