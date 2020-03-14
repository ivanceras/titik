use crossterm::event::EnableMouseCapture;
pub use crossterm::{
    cursor,
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
        MouseEvent,
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
    cell::RefCell,
    io::{
        self,
        Write,
    },
    rc::Rc,
};
use stretch::{
    geometry::{
        Rect,
        Size,
    },
    number::Number,
    style::{
        Dimension,
        FlexDirection,
        Style,
        *,
    },
};
use titik::{
    compute_layout,
    find_widget,
    find_widget_mut,
    widget_hit_at,
    widget_node_idx_at,
    Buffer,
    Button,
    Checkbox,
    FlexBox,
    Image,
    Radio,
    TextInput,
    Widget,
};

fn init<W: Write>(w: &mut W) -> Result<()> {
    execute!(w, terminal::EnterAlternateScreen)?;
    execute!(w, EnableMouseCapture)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

fn finalize<W: Write>(w: &mut W) -> Result<()> {
    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    init(w)?;

    let mut focused_widget_idx = None;
    let mut focused_widget = None;
    let mut buffer = String::new();
    let mut events = String::new();

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;

        let mut root_node = FlexBox::new();
        let (width, height) = buffer_size().unwrap();
        let mut cb1 = Checkbox::new(format!("{:?}", focused_widget_idx));
        cb1.set_checked(true);
        let mut cb2 = Checkbox::new("Checkbox2");
        cb2.set_checked(false);
        let mut rb1 = Radio::new("Radio1");
        rb1.set_checked(true);
        let mut input1 = TextInput::new("Hello world!");
        input1.set_value(&buffer);
        let rb2 = Radio::new("Radio2");
        let mut btn2 = Button::new(format!("Events: {}", events));
        btn2.set_rounded(true);
        let mut img = Image::new(include_bytes!("../horse.jpg").to_vec());
        img.set_size(Some(80.0), Some(40.0));
        root_node.set_size(Some((width - 2) as f32), Some(height as f32));
        root_node.vertical();

        let btn1 = Button::new(format!("{:?}", focused_widget));
        root_node.add_child(Box::new(btn1));
        root_node.add_child(Box::new(btn2));
        root_node.add_child(Box::new(img));
        root_node.add_child(Box::new(cb2));
        root_node.add_child(Box::new(cb1));

        root_node.add_child(Box::new(rb1));
        root_node.add_child(Box::new(rb2));
        root_node.add_child(Box::new(input1));

        focused_widget = focused_widget_idx
            .map(|idx| find_widget(&root_node, idx))
            .flatten()
            .map(|w| format!("{:?}", w));

        let layout_tree = compute_layout(
            &mut root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );

        if let Some(idx) = focused_widget_idx.as_ref() {
            if let Some(focused_widget) = find_widget_mut(&mut root_node, *idx)
            {
                focused_widget.set_focused(true);
            }
        }
        let mut buf = Buffer::new(width as usize, height as usize);
        root_node.draw(&mut buf, &layout_tree);
        write!(w, "{}", buf);
        w.flush()?;

        if let Ok(ev) = event::read() {
            events = format!("{:?}", ev);
            match ev {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers,
                }) => {
                    // To quite, press any of the following:
                    //  - CTRL-c
                    //  - CTRL-q
                    //  - CTRL-d
                    //  - CTRL-z
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        match c {
                            'c' | 'q' | 'd' | 'z' => break,
                            _ => (),
                        }
                    } else {
                        buffer.push(c);
                    }
                }
                Event::Mouse(MouseEvent::Down(btn, x, y, _modifier)) => {
                    focused_widget_idx =
                        widget_node_idx_at(&layout_tree, x as f32, y as f32);
                }
                _ => (),
            }
        }
    }
    finalize(w)?;
    Ok(())
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}
