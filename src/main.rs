use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    // --- 1. SETUP ---
    // Enter raw mode (reads input character-by-character)
    enable_raw_mode()?;
    let mut stdout = stdout();
    // Enter alternate screen (a separate buffer, so we don't mess up the user's history)
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // --- 2. MAIN LOOP ---
    let app_result = run_app(&mut terminal);

    // --- 3. CLEANUP ---
    // We must restore the terminal even if the app errors out
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Print error if the app crashed
    if let Err(err) = app_result {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| {
            let size = f.area();

            // Define a simple widget
            let paragraph = Paragraph::new("Welcome to Ratatui!\nPress 'q' to quit.")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(" Rat Race"));

            // Render the widget
            f.render_widget(paragraph, size);
        })?;

        // Handle Input
        if let Event::Key(key) = event::read()? {
            // Only react to key presses (not releases)
            if key.kind == KeyEventKind::Press {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}
