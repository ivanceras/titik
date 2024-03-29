//#![deny(warnings)]
use std::io::{self};
pub use titik::crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent},
    execute, queue, style,
    style::{Attribute, Attributes, Color, ContentStyle},
    terminal::{self, ClearType},
    Command, Result,
};

use titik::{
    Button, Callback, Checkbox, FlexBox, GroupBox, Image, Link, ListBox, Radio,
    Renderer, Slider, TabBox, TextArea, TextInput, Widget,
};

fn build_ui() -> Box<dyn Widget<()>> {
    let mut root_node = FlexBox::new();
    root_node.set_scroll_top(0.0);
    root_node.vertical();

    let mut slider = Slider::new(0.5);
    slider.use_thick_track(true);
    let mut tab1 = TabBox::new();
    tab1.set_tab_labels(vec![
        "Tab1".into(),
        "Tab2".into(),
        "Tab3".into(),
        "Tab4".into(),
        "Tab5".into(),
        "And more tabs..".into(),
    ]);
    tab1.set_active_tab(1);
    //tab1.set_size(None, Some(30.0));

    let mut gb1 = GroupBox::new();
    gb1.set_label("Selection");

    let cb1 = Checkbox::new("Checkbox1");
    let cb2 = Checkbox::new("Checkbox2");
    let rb1 = Radio::new("Radio1");
    let rb2 = Radio::new("Radio2");
    let mut link1 = Link::new("https://github.com", "Github");
    link1.set_border(true);

    let mut list_box1 = ListBox::new();
    list_box1.set_list(vec![
        "Item1".into(),
        "Item2".into(),
        "Item3".into(),
        "Item4".into(),
        "Item5".into(),
        "Item6".into(),
    ]);

    tab1.add_child_to_tab(0, Box::new(list_box1));

    gb1.add_child(Box::new(cb1));
    gb1.add_child(Box::new(cb2));
    gb1.add_child(Box::new(rb1));
    gb1.add_child(Box::new(rb2));
    gb1.add_child(Box::new(link1));

    let input1 = TextInput::new("Hello world!");

    let input2 =
        TextInput::new("The quick brown fox jumps over the lazy dog...");

    let mut text_area1: TextArea<()> = TextArea::new(
        "This is a text area\
            \n1. With a line that is a bit long.. but not very long....\
            \n2. and another line\
            \n3. With a veryyyyyyy looooooooooooooooooooooooooooooooooooonnnnnnnnnnngggggggg line\
            \n4. and another line...............\
            \n5. With a line\
            \n6. With a line and not too long\
            \n7. and another line\
            \n8. With a line\
            \n9. and another line\
            \n10. and another line with another pharse\
            \n11. With a line\
            \n12. and another line\
            \n13. With a line\
            \n14. and another line not so loooooooooooooooooooooooooong\
            \n15. With a line\
            \n16. With a line\
            \n17. and another line\
            \n18. With a line\
            \n19. and another line\
            \n20. This is the last line and also a looooooooooooooooooooong line",
    );
    //text_area1.set_size(Some(60.0), Some(10.0));

    let mut btn2: Button<()> = Button::new("Button2");
    btn2.set_rounded(true);
    btn2.set_id("btn2");
    let mut img: Image<()> =
        Image::new(include_bytes!("../horse.jpg").to_vec());

    let mut btn1: Button<()> = Button::new("Button 1");
    btn1.set_id("btn1");

    btn1.on_click(|_| {
        println!("btn1 is clicked");
    });

    root_node.add_child(Box::new(btn1));
    root_node.add_child(Box::new(slider));
    root_node.add_child(Box::new(btn2));
    tab1.add_child(Box::new(gb1));
    let mut row = FlexBox::new();
    row.horizontal();
    row.set_expand_width(true);
    row.set_expand_height(false);
    row.add_child(Box::new(img));
    root_node.add_child(Box::new(row));
    root_node.add_child(Box::new(tab1));
    //root_node.add_child(Box::new(gb1));
    root_node.add_child(Box::new(input1));
    root_node.add_child(Box::new(input2));
    root_node.add_child(Box::new(text_area1));
    Box::new(root_node)
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut root_node = build_ui();
    let mut renderer = Renderer::new(stdout, None, root_node.as_mut());
    renderer.run()?;
    Ok(())
}
