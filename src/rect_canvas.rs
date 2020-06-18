
/// canvas which only draws rectangluar shapes intended to be used
/// mainly in drawing borders of a widget
/// each location on this canvas can contain multiple character.
///
/// Upon flattening, each cell should resolve to only 1 char.
/// if all the chars one cells can be merge and resolve to a one
/// character then that char will be used, otherwise, the last inserted
/// char will be used
pub struct RectCanvas{
    cells: HashMap<(usize, usize), Vec<char>),
}
