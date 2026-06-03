use crate::app::{App, HistoryEntry, OutputBase};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

//formats a historyentry into a listelement
fn make_item(app: &App, e: &HistoryEntry) -> ListItem<'static>
{
    match &e.result//format resultstring based on outputmode color based on type
        {
            Ok(n) => match app.output_mode
            {
                OutputBase::Decimal =>ListItem::new(format!("{}", n)),
                OutputBase::Hex => ListItem::new(format!("{:#X}", *n as i64)),
                OutputBase::Binary => ListItem::new(format!("{:#b}", *n as i64)),
            },
            Err(e) => ListItem::new(format!("{:?}", e)).style(Style::default().fg(Color::Red))
        }
}

//draw the application frame
pub fn draw(frame: &mut Frame, app: &App)
{
    //split terminal into sections (chunks)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)]) //status bar, main window
        .split(frame.area());

    //draw main window
    //create list with all historyentries and current input
    let mut items: Vec<ListItem> = Vec::new();
    for i in 0..app.selected.unwrap_or(app.history.len())
    //add all historyentries before currently selected input (cursor position)
    {
        let e = &app.history[i];

        //create item
        items.push(make_item(app, e));
    }

    //add current input (cursor position)
    items.push(ListItem::new(app.input.as_str()));

    //add all historyentries after current input
    for i in app.selected.unwrap_or(app.history.len()) + 1..app.history.len()
    {
        let e = &app.history[i];
        //create item
        items.push(make_item(app, e));
    }

    //define main window with listitems
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL) //border type
            .border_style(Style::default().fg(Color::Rgb(122, 121, 28))), //border color
    );

    frame.render_widget(list, chunks[1]); //render main window
    //render cursor
    frame.set_cursor_position(ratatui::layout::Position {
        x: app.cursor as u16 + 1,
        y: app.selected.unwrap_or(app.history.len()) as u16 + 2,
    });

    //define status bar
    let status = match app.output_mode
    {
        OutputBase::Decimal => "[dec]",
        OutputBase::Hex => "[hex]",
        OutputBase::Binary => "[bin]",
    };

    frame.render_widget(Paragraph::new(status), chunks[0]); //render status bar
}
