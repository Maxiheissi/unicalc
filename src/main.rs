use crossterm::
{
    event::{self, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::
{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal, Frame,
};
use std::io::{stdout, Result};

fn main() -> Result<()>
{
    // --- 1. Terminal Setup ---
    // Switch to the alternate screen (so the terminal clears on start)
    stdout().execute(EnterAlternateScreen)?;
    // Enable raw mode to capture keyboard input immediately
    enable_raw_mode()?;
    
    // Initialize the Ratatui terminal with the Crossterm backend
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Application State
    let mut input = String::new();
    let mut history: Vec<String> = Vec::new();

    // --- 2. Main Application Loop ---
    loop
    {
        // Render the user interface
        terminal.draw(|f| draw_ui(f, &input, &history))?;

        // Check for terminal events (keyboard, resize, etc.)
        if event::poll(std::time::Duration::from_millis(16))?
        {
            if let event::Event::Key(key) = event::read()?
            {
                // Only process the event if a key was pressed (ignores release events)
                if key.kind == KeyEventKind::Press
                {
                    // Pass input logic to our handler
                    let should_quit = handle_input(key, &mut input, &mut history);
                    
                    // Exit the loop if the handler returns true
                    if should_quit
                    {
                        break;
                    }
                }
            }
        }
    }

    // --- 3. Cleanup ---
    // Restore the terminal to its original state before exiting
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Processes keyboard input and updates the application state.
/// Returns true if the application should exit.
fn handle_input(key: KeyEvent, input: &mut String, history: &mut Vec<String>) -> bool
{
    match key.code
    {
        // Quit the application
        KeyCode::Char('q') =>
        {
            return true;
        }
        
        // Append characters to the current input string
        KeyCode::Char(c) =>
        {
            input.push(c);
        }
        
        // Remove the last character
        KeyCode::Backspace =>
        {
            input.pop();
        }
        
        // Submit the calculation to history
        KeyCode::Enter =>
        {
            if !input.is_empty()
            {
                history.push(format!("{}", input));
                input.clear();
            }
        }
        
        _ => {}
    }
    false
}

/// Renders the UI widgets into the terminal frame.
fn draw_ui(f: &mut Frame, input: &str, history: &[String])
{
    // Define vertical layout sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Top: History (grows to fill space)
            Constraint::Length(3), // Middle: Input field (fixed height)
            Constraint::Length(1), // Bottom: Status/Help bar
        ])
        .split(f.area());

    // 1. History Widget (shows previous inputs)
    f.render_widget(
        Paragraph::new(history.join("\n"))
            .block(Block::default().title(" unicalc ").borders(Borders::ALL)),
        chunks[0],
    );

    // 2. Input Widget (shows what the user is currently typing)
    f.render_widget(
        Paragraph::new(input)
            .block(Block::default().title(" Input ").borders(Borders::ALL)),
        chunks[1],
    );

    // 3. Footer Widget (simple help text without borders)
    f.render_widget(
        Paragraph::new("[Q] quit | [Enter] submit")
            .block(Block::default().borders(Borders::NONE)),
        chunks[2],
    );
}