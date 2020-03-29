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
    fmt,
    io::{
        self,
        Write,
    },
    rc::Rc,
};

use titik::{
    command,
    compute_layout,
    find_layout,
    find_widget,
    find_widget_mut,
    renderer,
    renderer::Renderer,
    set_focused_node,
    widget_hit_at,
    widget_node_idx_at,
    Buffer,
    Button,
    Callback,
    Checkbox,
    Cmd,
    FlexBox,
    Image,
    InputBuffer,
    LayoutTree,
    Radio,
    SvgImage,
    TextArea,
    TextInput,
    Widget,
};

fn build_ui<MSG>() -> Box<dyn Widget<MSG>>
where
    MSG: fmt::Debug + 'static,
{
    let mut root_node = FlexBox::new();
    let cb1 = Checkbox::new("Checkbox1");
    let cb2 = Checkbox::new("Checkbox2");
    let rb1 = Radio::new("Radio1");
    let input1 = TextInput::new("Hello world!");

    let input2 =
        TextInput::new("The quick brown fox jumps over the lazy dog...");

    let mut text_area1: TextArea<MSG> = TextArea::new(
        "This is a text area\
            \n1. With a line\
            \n2. and another line\
            \n3. With a line\
            \n4. and another line\
            \n5. With a line\
            \n6. With a line\
            \n7. and another line\
            \n8. With a line\
            \n9. and another line\
            \n10. and another line\
            \n11. With a line\
            \n12. and another line\
            \n13. With a line\
            \n14. and another line\
            \n15. With a line\
            \n16. With a line\
            \n17. and another line\
            \n18. With a line\
            \n19. and another line\
            \n",
    );
    text_area1.set_size(None, Some(7.0));

    let rb2 = Radio::new("Radio2");
    let mut btn2: Button<MSG> = Button::new("Button2");
    btn2.set_rounded(true);
    let mut img: Image<MSG> =
        Image::new(include_bytes!("../horse.jpg").to_vec());
    img.set_size(Some(60.0), Some(20.0));

    let svg: SvgImage<MSG> =
        SvgImage::new(include_str!("../tiger.svg").to_string());
    root_node.vertical();

    let btn1: Button<MSG> = Button::new("Button 1");
    root_node.add_child(Box::new(btn1));
    root_node.add_child(Box::new(btn2));
    let mut row = FlexBox::new();
    row.horizontal();
    row.add_child(Box::new(img));
    row.add_child(Box::new(svg));
    root_node.add_child(Box::new(row));
    root_node.add_child(Box::new(cb2));
    root_node.add_child(Box::new(cb1));

    root_node.add_child(Box::new(rb1));
    root_node.add_child(Box::new(rb2));
    root_node.add_child(Box::new(input1));
    root_node.add_child(Box::new(input2));
    root_node.add_child(Box::new(text_area1));
    Box::new(root_node)
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut root_node: Box<dyn Widget<()>> = build_ui();
    let mut renderer = Renderer::new(&mut stdout, None, root_node);
    renderer.run()
}

