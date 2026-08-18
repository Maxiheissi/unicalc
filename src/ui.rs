use crate::app::{App, HistoryEntry, OutputBase};
use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::{
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

//formats a historyentry into a listelement
fn make_item(app: &App, e: &HistoryEntry) -> ListItem<'static> {
    match &e.result {
        Ok(n) => match app.output_mode {
            OutputBase::Decimal => ListItem::new(format!("{}", n)),
            OutputBase::Hex => ListItem::new(format!("{:#X}", *n as i64)),
            OutputBase::Binary => ListItem::new(format!("{:#b}", *n as i64)),
        },
        Err(e) => ListItem::new(format!("{:?}", e)).style(Style::default().fg(Color::Red)),
    }
}

//draw the application frame
pub fn draw(frame: &mut Frame, app: &App) {
    // 1. First Split: Vertical for status bar (top) and content area (bottom)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.area());

    let status_area = vertical_chunks[0];
    let content_area = vertical_chunks[1];

    // 2. Second Split: Horizontal split for content area (only if help is toggled)
    let main_areas = if app.show_help {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_area)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(content_area)
    };

    let calc_area = main_areas[0];

    //  Render Status Bar 
let status_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
        Constraint::Min(10),
                Constraint::Length(15),  
    ])
    .split(status_area);

let status = match app.output_mode {
    OutputBase::Decimal => "[DEC]",
    OutputBase::Hex => "[HEX]",
    OutputBase::Binary => "[BIN]",
};

frame.render_widget(Paragraph::new(status).style(Style::default().fg(Color::DarkGray)), status_chunks[0]);

let help_hint = Paragraph::new("[F2 Help]")
    .style(Style::default().fg(Color::DarkGray))
    .alignment(ratatui::layout::Alignment::Right);

frame.render_widget(help_hint, status_chunks[1]);
    // Draw Main Window
    let mut items: Vec<ListItem> = Vec::new();
    for i in 0..app.selected.unwrap_or(app.history.len()) {
        let e = &app.history[i];
        items.push(make_item(app, e));
    }

    //add current input
    items.push(ListItem::new(app.input.as_str()));

    //add all historyentries after current input
    for i in app.selected.unwrap_or(app.history.len()) + 1..app.history.len() {
        let e = &app.history[i];
        items.push(make_item(app, e));
    }

    //define main window with listitems
    let list = List::new(items).block(
        Block::default()
            .title(" main ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(122, 121, 28))),
    );

    frame.render_widget(list, calc_area);

    //render cursor
    frame.set_cursor_position(Position {
        x: calc_area.x + app.cursor as u16 + 1,
        y: calc_area.y + app.selected.unwrap_or(app.history.len()) as u16 + 1,
    });

    // 5. Draw English Help Page when toggled
    if app.show_help && main_areas.len() > 1 {
        let help_area = main_areas[1];

        let help_text = vec![
            Line::from(Span::styled("--- Keyboard Shortcuts ---", Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from(vec![Span::styled("  Enter        ", Style::default().fg(Color::Cyan)), Span::raw("Evaluate expression")]),
            Line::from(vec![Span::styled("  ← / →        ", Style::default().fg(Color::Cyan)), Span::raw("Move cursor")]),
            Line::from(vec![Span::styled("  ↑ / ↓        ", Style::default().fg(Color::Cyan)), Span::raw("Navigate history")]),
            Line::from(vec![Span::styled("  Ctrl + D     ", Style::default().fg(Color::Cyan)), Span::raw("Delete entry")]),
            Line::from(vec![Span::styled("  F1           ", Style::default().fg(Color::Cyan)), Span::raw("Toggle format (DEC/HEX/BIN)")]),
            Line::from(vec![Span::styled("  F2           ", Style::default().fg(Color::Cyan)), Span::raw("Toggle help page")]),
            Line::from(vec![Span::styled("  ESC          ", Style::default().fg(Color::Cyan)), Span::raw("Quit application")]),
            Line::from(""),
           Line::from(Span::styled("--- Functions ---", Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from(vec![Span::styled("  sqrt(x)      ", Style::default().fg(Color::Green)), Span::raw("Square root")]),
            Line::from(vec![Span::styled("  abs(x)       ", Style::default().fg(Color::Green)), Span::raw("Absolute value")]),
            Line::from(vec![Span::styled("  sin(x)       ", Style::default().fg(Color::Green)), Span::raw("Sine")]),
            Line::from(vec![Span::styled("  cos(x)       ", Style::default().fg(Color::Green)), Span::raw("Cosine")]),
            Line::from(vec![Span::styled("  tan(x)       ", Style::default().fg(Color::Green)), Span::raw("Tangent")]),
            Line::from(vec![Span::styled("  asin(x)      ", Style::default().fg(Color::Green)), Span::raw("Arc sine")]),
            Line::from(vec![Span::styled("  acos(x)      ", Style::default().fg(Color::Green)), Span::raw("Arc cosine")]),
            Line::from(vec![Span::styled("  atan(x)      ", Style::default().fg(Color::Green)), Span::raw("Arc tangent")]),
            Line::from(vec![Span::styled("  ln(x)        ", Style::default().fg(Color::Green)), Span::raw("Natural logarithm")]),
            Line::from(vec![Span::styled("  log10(x)     ", Style::default().fg(Color::Green)), Span::raw("Base-10 logarithm")]),
            Line::from(""),
            Line::from(Span::styled("--- Constants ---", Style::default().fg(Color::Yellow))),
            Line::from(""),
            Line::from(vec![Span::styled("  pi           ", Style::default().fg(Color::Green)), Span::raw("3.14159...")]),
            Line::from(vec![Span::styled("  e            ", Style::default().fg(Color::Green)), Span::raw("2.71828...")]),
        ];
        let help_widget = Paragraph::new(help_text)
            .block(
                Block::default()
                    .title(" help ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Rgb(122, 121, 28))),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(help_widget, help_area);
    }
}
