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
    find_layout,
    find_widget,
    find_widget_mut,
    set_focused_node,
    widget_hit_at,
    widget_node_idx_at,
    Buffer,
    Button,
    Checkbox,
    Cmd,
    FlexBox,
    Image,
    InputBuffer,
    LayoutTree,
    Radio,
    TextArea,
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
    w.flush()?;
    Ok(())
}

fn clear<W: Write>(w: &mut W) -> crossterm::Result<()> {
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::Show,
        cursor::MoveTo(1, 1)
    )?;
    Ok(())
}

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    init(w)?;

    let mut focused_widget_idx = None;

    let mut root_node = FlexBox::new();
    let mut cb1 = Checkbox::new("Checkbox1");
    let mut cb2 = Checkbox::new("Checkbox2");
    let mut rb1 = Radio::new("Radio1");
    let mut input1 = TextInput::new("Hello world!");

    let mut input2 =
        TextInput::new("The quick brown fox jumps over the lazy dog...");

    let mut text_area1: TextArea<()> =
        TextArea::new("This is a text area\
            \nWith a line\
            \nand another line\
            \nWith a line\
            \nand another line\
            \n");
    text_area1.set_size(None, Some(5.0));
    text_area1.scroll = 3;

    let rb2 = Radio::new("Radio2");
    let mut btn2: Button<()> = Button::new("Button2");
    btn2.set_rounded(true);
    let mut img: Image<()> =
        Image::new(include_bytes!("../horse.jpg").to_vec());
    img.set_size(Some(60.0), Some(20.0));
    root_node.vertical();

    let btn1: Button<()> = Button::new("Button 1");
    root_node.add_child(Box::new(btn1));
    root_node.add_child(Box::new(btn2));
    root_node.add_child(Box::new(img));
    root_node.add_child(Box::new(cb2));
    root_node.add_child(Box::new(cb1));

    root_node.add_child(Box::new(rb1));
    root_node.add_child(Box::new(rb2));
    root_node.add_child(Box::new(input1));
    root_node.add_child(Box::new(input2));
    root_node.add_child(Box::new(text_area1));

    let (width, height) = buffer_size().unwrap();

    loop {
        let (width, height) = buffer_size().unwrap();
        root_node.set_size(Some((width) as f32), Some(height as f32));
        let layout_tree = compute_layout(
            &mut root_node,
            Size {
                width: Number::Defined(width as f32),
                height: Number::Defined(height as f32),
            },
        );

        let mut buf = Buffer::new(width as usize, height as usize);
        buf.reset();
        let cmds = root_node.draw(&mut buf, &layout_tree);
        buf.render(w);
        cmds.iter()
            .for_each(|cmd| cmd.execute(w).expect("must execute"));
        w.flush();

        if let Ok(event) = event::read() {
            match event {
                Event::Key(key_event) => {
                    // To quite, press any of the following:
                    //  - CTRL-c
                    //  - CTRL-q
                    //  - CTRL-d
                    //  - CTRL-z
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        match key_event.code {
                            KeyCode::Char(c) => {
                                match c {
                                    'c' | 'q' | 'd' | 'z' => {
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    } else {
                        if let Some(idx) = focused_widget_idx.as_ref() {
                            let active_widget: Option<&mut dyn Widget<()>> =
                                find_widget_mut(&mut root_node, *idx);
                            let focused_layout =
                                find_layout(&layout_tree, *idx)
                                    .expect("must have a layout tree");
                            if let Some(focused_widget) = active_widget {
                                focused_widget.process_event(
                                    event,
                                    &focused_layout.layout,
                                );
                            }
                        }
                    }
                }
                Event::Mouse(MouseEvent::Down(btn, x, y, _modifier)) => {
                    focused_widget_idx =
                        widget_node_idx_at(&layout_tree, x as f32, y as f32);

                    if let Some(idx) = focused_widget_idx.as_ref() {
                        set_focused_node(&mut root_node, *idx);
                    }
                }
                _ => (),
            }
            if let Some((x, y)) = extract_location(&event) {
                let mut hits = layout_tree.hit(x as f32, y as f32);
                let hit = hits.pop().expect("process only 1 for now");
                    let mut hit_widget: Option<&mut dyn Widget<()>> =
                        find_widget_mut(&mut root_node, hit);

                    let focused_layout = find_layout(&layout_tree, hit)
                        .expect("must have a layout tree");

                    if let Some(hit_widget) = &mut hit_widget {
                        hit_widget.process_event(event, &focused_layout.layout);
                    }
            }
        }
    }
    finalize(w)?;
    Ok(())
}

fn extract_location(event: &Event) -> Option<(u16, u16)> {
    match event {
        Event::Key(_) => None,
        Event::Mouse(MouseEvent::Down(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::Up(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::Drag(_btn, x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::ScrollDown(x, y, _modifier)) => Some((*x, *y)),
        Event::Mouse(MouseEvent::ScrollUp(x, y, _modifier)) => Some((*x, *y)),
        Event::Resize(_, _) => None,
    }
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}
