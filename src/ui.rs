use crate::app::{App, OutputBase};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App)
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Statusleiste
            Constraint::Min(1),    // Liste
        ])
        .split(frame.area());

    let mut items: Vec<ListItem> = Vec::new();
    for i in 0..app.selected.unwrap_or(app.history.len())
    {
        let e = &app.history[i];

        let result_str = match &e.result
        {
            Ok(n) => match app.output_mode
            {
                OutputBase::Decimal => format!("{}", n),
                OutputBase::Hex => format!("{:#X}", *n as i64),
                OutputBase::Binary => format!("{:#b}", *n as i64),
            },
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

    frame.render_widget(list, chunks[1]);
    frame.set_cursor_position(ratatui::layout::Position {
        x: app.cursor as u16 + 1,
        y: app.selected.unwrap_or(app.history.len()) as u16 + 2,
    });

    let status = match app.output_mode
    {
        OutputBase::Decimal => "[dec]",
        OutputBase::Hex => "[hex]",
        OutputBase::Binary => "[bin]",
    };

    frame.render_widget(Paragraph::new(status), chunks[0]);
}
