use std::io;
use titik::crossterm::Result;
use titik::{
    Button, Callback, Checkbox, FlexBox, GroupBox, Image, ListBox, Radio,
    Renderer, Slider, SvgImage, TabBox, TextArea, TextInput, Widget,
};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut root_node = FlexBox::new();
    let mut column = FlexBox::new();
    column.vertical();
    let mut row = FlexBox::new();
    row.horizontal();
    let btn1 = Button::new("btn 1");
    let btn2 = Button::new("btn 2");
    let cb1 = Checkbox::new("cb 1");
    let cb2 = Checkbox::new("cb 2");
    row.add_child(Box::new(btn1));
    row.add_child(Box::new(btn2));
    column.add_child(Box::new(cb1));
    column.add_child(Box::new(cb2));
    column.add_child(Box::new(row));
    root_node.add_child(Box::new(column));
    let mut renderer = Renderer::<()>::new(&mut stdout, None, &mut root_node);
    renderer.run()?;
    Ok(())
}
