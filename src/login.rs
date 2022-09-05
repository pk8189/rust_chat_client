use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

enum InputMode {
    Username,
    Password,
}

pub struct Login {
    username: String,
    password: String,
    input_mode: InputMode,
}

impl Default for Login {
    fn default() -> Login {
        Login {
            username: String::new(),
            password: String::new(),
            input_mode: InputMode::Username,
        }
    }
}

pub fn run_login<B: Backend>(terminal: &mut Terminal<B>, mut app: Login) -> io::Result<()> {
    loop {
        terminal.draw(|f| login_ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Username => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Password;
                    }
                    KeyCode::Char(c) => {
                        app.username.push(c);
                    }
                    KeyCode::Backspace => {
                        app.username.pop();
                    }
                    KeyCode::Esc => {
                        if app.username.len() == 0 {
                            return Ok(());
                        } else {
                            app.username.drain(..);
                        }
                    }
                    _ => {}
                },
                InputMode::Password => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Username;
                    }
                    KeyCode::Char(c) => {
                        app.password.push(c);
                    }
                    KeyCode::Backspace => {
                        app.password.pop();
                    }
                    KeyCode::Esc => {
                        if app.password.len() == 0 {
                            return Ok(());
                        } else {
                            app.password.drain(..);
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}

fn login_ui<B: Backend>(f: &mut Frame<B>, app: &Login) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
        .split(f.size());

    let username_input = Paragraph::new(app.username.as_ref())
        .style(match app.input_mode {
            InputMode::Password => Style::default(),
            InputMode::Username => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Username"));
    f.render_widget(username_input, chunks[0]);

    let password_input = Paragraph::new(app.password.as_ref())
        .style(match app.input_mode {
            InputMode::Username => Style::default(),
            InputMode::Password => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Password"));
    f.render_widget(password_input, chunks[1]);
}
