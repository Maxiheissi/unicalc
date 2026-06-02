use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;
mod app;
mod eval;
mod ui;

fn main() -> Result<(), io::Error>
{
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        disable_raw_mode().unwrap();
        execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        original_hook(info);
    }));

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new();
    loop
    {
        terminal.draw(|f| ui::draw(f, &app))?;

        match event::read()?
        {
            Event::Key(KeyEvent { code, .. }) => match code
            {
                KeyCode::Char(c) => app.push_input(c),
                KeyCode::Backspace => app.pop_input(),
                KeyCode::Enter => app.evaluate(),
                KeyCode::Left => app.cursor_left(),
                KeyCode::Right => app.cursor_right(),
                KeyCode::Up => app.selected_up(),
                KeyCode::Down => app.selected_down(),
                KeyCode::Esc => break,
                KeyCode::F(1) => app.cycle_output_mode(),
                _ =>
                {}
            },
            _ =>
            {}
        }
    }
    execute!(io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
