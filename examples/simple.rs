use std::io;
use titik::crossterm::Result;
use titik::{
    Button, Callback, Checkbox, FlexBox, GroupBox, Image, ListBox, Radio,
    Renderer, Slider, SvgImage, TabBox, TextArea, TextInput, Widget,
};

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    let mut root_node = FlexBox::<()>::new();
    root_node.set_border(false);
    root_node.set_thick_border(true);
    let mut column = FlexBox::new();
    column.vertical();
    column.set_border(false);
    column.set_thick_border(false);
    let mut row = FlexBox::<()>::new();
    row.horizontal();
    row.set_border(false);
    row.set_thick_border(false);
    row.set_rounded(true);
    let btn1 = Button::<()>::new("btn 1");
    let btn2 = Button::<()>::new("btn 2");
    let cb1 = Checkbox::<()>::new("cb 1");
    let cb2 = Checkbox::<()>::new("cb 2");

    let rb1 = Radio::<()>::new("Radio1");
    let rb2 = Radio::<()>::new("Radio2");

    let input1 = TextInput::new("Hello");

    let mut list_box1 = ListBox::<()>::new();
    list_box1.set_use_divider(true);
    list_box1.set_list(vec![
        "Item1".into(),
        "Item2".into(),
        "Item3".into(),
        "Item4".into(),
        "Item5".into(),
        "Item6".into(),
        "Item7".into(),
        "Item8".into(),
        "Item9".into(),
        "Item10".into(),
        "Item11".into(),
        "Item12".into(),
        "Item13".into(),
        "Item14".into(),
        "Item15".into(),
        "Item16".into(),
        "Item17".into(),
        "Item18".into(),
        "Item19".into(),
        "Item20".into(),
        "Item21".into(),
        "Item22".into(),
        "Item23".into(),
        "Item24".into(),
        "Item25".into(),
        "Item26".into(),
        "Item27".into(),
        "Item28".into(),
        "Item29".into(),
        "Item30".into(),
    ]);

    let mut tab1 = TabBox::new();
    tab1.set_tab_labels(vec![
        "Tab1".into(),
        "Tab2".into(),
        "Tab3".into(),
        "Tab4".into(),
        "Tab5".into(),
        "And more tabs..".into(),
    ]);

    let mut tab_row = FlexBox::new();
    tab_row.vertical();

    tab1.set_active_tab(1);
    tab1.add_child_to_tab(1, Box::new(list_box1));
    tab_row.add_child(Box::new(Checkbox::new("checkbox1 in first tab")));
    tab_row.add_child(Box::new(Checkbox::new("checkbox2 in first tab")));
    tab_row.add_child(Box::new(Radio::new("radio1 in first tab")));
    tab_row.add_child(Box::new(TextInput::new("Hello input in first tab")));
    tab1.add_child_to_tab(0, Box::new(tab_row));

    column.add_child(Box::new(cb1));
    column.add_child(Box::new(cb2));
    column.add_child(Box::new(rb1));
    column.add_child(Box::new(rb2));
    column.add_child(Box::new(input1));

    row.add_child(Box::new(btn1));
    row.add_child(Box::new(btn2));
    row.add_child(Box::new(tab1));
    //row.add_child(Box::new(list_box1));
    column.add_child(Box::new(row));
    root_node.add_child(Box::new(column));
    let mut renderer = Renderer::<()>::new(&mut stdout, None, &mut root_node);
    renderer.run()?;
    Ok(())
}
