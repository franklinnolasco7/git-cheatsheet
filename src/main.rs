mod app;
mod data;
mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use app::{App, Mode, Pane};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

const TICK_RATE_MS: u64 = 100;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_default();

        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            match app.mode {
                Mode::Help => match key.code {
                    KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::Esc => {
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    _ => {}
                },
                Mode::Search => match key.code {
                    KeyCode::Esc => app.exit_search(),
                    KeyCode::Backspace => app.remove_search_character(),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.select_next_command(),
                    KeyCode::Up | KeyCode::Char('k') => app.select_previous_command(),
                    KeyCode::Enter | KeyCode::Char('y') => {
                        if let Some(text) = app.extract_command_for_clipboard() {
                            let _ = copy_to_clipboard(&text);
                        }
                    }
                    KeyCode::Char(c) => app.append_search_character(c),
                    _ => {}
                },
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
                    KeyCode::Char('?') => app.mode = Mode::Help,
                    KeyCode::Char('/') => app.enter_search(),
                    KeyCode::Down | KeyCode::Char('j') => match app.active_pane {
                        Pane::List => app.select_next_command(),
                        Pane::Sidebar => app.select_next_category(),
                    },
                    KeyCode::Up | KeyCode::Char('k') => match app.active_pane {
                        Pane::List => app.select_previous_command(),
                        Pane::Sidebar => app.select_previous_category(),
                    },
                    KeyCode::Right | KeyCode::Char('l') => app.select_next_category(),
                    KeyCode::Left | KeyCode::Char('h') => app.select_previous_category(),
                    KeyCode::Char('g') => app.command_index = 0,
                    KeyCode::Char('G') => {
                        let len = app.visible_commands().len();
                        if len > 0 {
                            app.command_index = len - 1;
                        }
                    }
                    KeyCode::Tab => {
                        app.active_pane = match app.active_pane {
                            Pane::List => Pane::Sidebar,
                            Pane::Sidebar => Pane::List,
                        };
                    }
                    KeyCode::Enter | KeyCode::Char('y') => {
                        if let Some(text) = app.extract_command_for_clipboard() {
                            let _ = copy_to_clipboard(&text);
                        }
                    }
                    _ => {}
                },
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update_timers();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let clipboard_tools: &[(&str, &[&str])] = &[
        ("xclip", &["-selection", "clipboard"]),
        ("xsel", &["--clipboard", "--input"]),
        ("wl-copy", &[]),
        ("pbcopy", &[]),
    ];

    for (tool, args) in clipboard_tools {
        let Ok(mut child) = Command::new(tool).args(*args).stdin(Stdio::piped()).spawn() else {
            continue;
        };
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        return Ok(());
    }

    Ok(())
}
