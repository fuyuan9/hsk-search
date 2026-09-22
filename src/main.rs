#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::too_many_lines
)]

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
        MouseEvent, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use hsk_search::app::{App, AppMode};
use hsk_search::load_dataset;
use hsk_search::ui;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};
use std::time::Duration;

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(out);
    Terminal::new(backend)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn set_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

fn handle_key_event(app: &mut App, key: crossterm::event::KeyEvent) {
    if key.kind != event::KeyEventKind::Press {
        return;
    }

    // Universal shortcuts with Ctrl modifier (active in both modes)
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c' | 'q') => {
                app.should_quit = true;
                return;
            }
            KeyCode::Char('d') => {
                app.page_down(8);
                return;
            }
            KeyCode::Char('u') => {
                if app.mode == AppMode::Normal {
                    app.page_up(8);
                } else {
                    app.clear_search();
                }
                return;
            }
            KeyCode::Char('j' | 'n') => {
                app.next_item();
                return;
            }
            KeyCode::Char('k' | 'p') => {
                app.previous_item();
                return;
            }
            KeyCode::Char('w') => {
                app.delete_word_backward();
                return;
            }
            KeyCode::Char('h') => {
                app.delete_char_before_cursor();
                return;
            }
            KeyCode::Char('a') => {
                app.move_cursor_home();
                return;
            }
            KeyCode::Char('e') => {
                app.move_cursor_end();
                return;
            }
            KeyCode::Char('f') => {
                app.page_down(15);
                return;
            }
            KeyCode::Char('b') => {
                app.page_up(15);
                return;
            }
            _ => {}
        }
    }

    // Modal dismiss for Help popup
    if app.show_help {
        match key.code {
            KeyCode::Esc | KeyCode::F(2) | KeyCode::Char('?' | 'q') => {
                app.show_help = false;
            }
            _ => {}
        }
        return;
    }

    match app.mode {
        AppMode::Insert => handle_insert_mode_key(app, key.code),
        AppMode::Normal => handle_normal_mode_key(app, key.code),
    }
}

fn handle_insert_mode_key(app: &mut App, code: KeyCode) {
    match code {
        // Vim escape: drop into Normal mode
        KeyCode::Esc | KeyCode::Enter => {
            app.enter_normal();
        }
        KeyCode::Tab => {
            app.next_level();
        }
        KeyCode::BackTab => {
            app.prev_level();
        }
        KeyCode::F(1) => {
            app.next_kind();
        }
        KeyCode::F(2) => {
            app.toggle_help();
        }
        KeyCode::Up => {
            app.previous_item();
        }
        KeyCode::Down => {
            app.next_item();
        }
        KeyCode::PageUp => {
            app.page_up(10);
        }
        KeyCode::PageDown => {
            app.page_down(10);
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Home => {
            app.move_cursor_home();
        }
        KeyCode::End => {
            app.move_cursor_end();
        }
        KeyCode::Backspace => {
            app.delete_char_before_cursor();
        }
        KeyCode::Delete => {
            app.delete_char_at_cursor();
        }
        KeyCode::Char(c) => {
            app.insert_char(c);
        }
        _ => {}
    }
}

fn handle_normal_mode_key(app: &mut App, code: KeyCode) {
    // Check if previous key was 'g'
    let was_g = app.last_normal_key == Some('g');
    app.last_normal_key = None;

    match code {
        // Navigation: hjkl
        KeyCode::Char('j') | KeyCode::Down => {
            app.next_item();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.previous_item();
        }
        KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
            app.prev_level();
        }
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
            app.next_level();
        }

        // Jump to top: gg
        KeyCode::Char('g') => {
            if was_g {
                app.jump_to_top();
            } else {
                app.last_normal_key = Some('g');
            }
        }

        // Jump to bottom: G
        KeyCode::Char('G') => {
            app.jump_to_bottom();
        }

        // Page scrolling
        KeyCode::PageUp => {
            app.page_up(10);
        }
        KeyCode::PageDown => {
            app.page_down(10);
        }

        // Switching to Insert mode
        KeyCode::Char('i' | '/') => {
            app.enter_insert();
        }
        KeyCode::Char('a') => {
            app.enter_insert_append();
        }
        KeyCode::Char('I') => {
            app.enter_insert_start();
        }
        KeyCode::Char('A') => {
            app.enter_insert_end();
        }
        KeyCode::Char('c' | 'C' | 's' | 'S') => {
            app.enter_insert_clear();
        }

        // Filters and Help
        KeyCode::Char('K' | 't') | KeyCode::F(1) => {
            app.next_kind();
        }
        KeyCode::Char('?' | 'H') | KeyCode::F(2) => {
            app.toggle_help();
        }

        // Application exit
        KeyCode::Char('q') => {
            app.should_quit = true;
        }

        _ => {}
    }
}

fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            app.handle_mouse_click(mouse.column, mouse.row);
        }
        MouseEventKind::ScrollDown => {
            app.next_item();
        }
        MouseEventKind::ScrollUp => {
            app.previous_item();
        }
        _ => {}
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    handle_key_event(&mut app, key);
                }
                Event::Mouse(mouse) => {
                    handle_mouse_event(&mut app, mouse);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    set_panic_hook();

    let items =
        load_dataset().map_err(|e| format!("Failed to parse embedded HSK 3.0 dataset: {e}"))?;

    let app = App::new(items);

    let mut terminal = setup_terminal()?;
    let run_res = run_app(&mut terminal, app);
    restore_terminal(terminal)?;

    if let Err(err) = run_res {
        eprintln!("Application error: {err}");
    }

    Ok(())
}
