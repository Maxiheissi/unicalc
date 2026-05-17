use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App)
{
    let mut items: Vec<ListItem> = Vec::new();
    for i in 0..app.selected.unwrap_or(app.history.len())
    {
        let e = &app.history[i];

        let result_str = match &e.result
        {
            Ok(n) => format!("{}", n),
            Err(e) => format!("{:?}", e),
        };
        items.push(ListItem::new(format!("{} = {}", e.input, result_str)));
    }

    items.push(ListItem::new(app.input.as_str()));

    for i in app.selected.unwrap_or(app.history.len()) + 1..app.history.len()
    {
        let e = &app.history[i];

        let result_str = match &e.result
        {
            Ok(n) => format!("{}", n),
            Err(e) => format!("{:?}", e),
        };
        items.push(ListItem::new(format!("{} = {}", e.input, result_str)));
    }

    let list = List::new(items).block(Block::default().borders(Borders::ALL));

    frame.render_widget(list, frame.area());
    frame.set_cursor_position(ratatui::layout::Position {
        x: app.cursor as u16 + 1,
        y: app.selected.unwrap_or(app.history.len()) as u16 + 1,
    });
}
