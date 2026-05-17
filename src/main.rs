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

// // program entry
// fn main()
// {
//     let input = "(-6+2)*sin(1)%2";
//     let input = "0b1010+0b1";
//     let tokens = eval::tokenize_expression(input);

//     let mut it = tokens.into_iter().peekable();
//     match eval::parse_expression(&mut it)
//     {
//         Ok(root) =>
//         {
//             eval::print_tree(&root, 0);
//             match eval::eval_tree(&root)
//             {
//                 Ok(result) => println!("result: {}", result),
//                 Err(e) => println!("eval error: {:?}", e),
//             }
//         }
//         Err(e) => println!("parse error: {:?}", e),
//     }
// }
