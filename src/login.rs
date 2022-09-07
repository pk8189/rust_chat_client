use crossterm::event::{self, Event, KeyCode};
use reqwest;
use serde::Deserialize;
use std::{collections::HashMap, io};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(Deserialize)]
struct LoginResponse {
    pub token: String,
}

enum InputMode {
    Username,
    Password,
}

pub struct Login {
    username: String,
    password: String,
    input_mode: InputMode,
    error_msg: String,
}

impl Default for Login {
    fn default() -> Login {
        Login {
            username: String::new(),
            password: String::new(),
            input_mode: InputMode::Username,
            error_msg: String::new(),
        }
    }
}

fn get_token(username: &str, password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("username", username);
    map.insert("password", password);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("http://localhost:8000/login")
        .json(&map)
        .send()?;

    if resp.status().is_success() {
        let res: LoginResponse = resp.json()?;
        return Ok(res.token);
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unable to log in with the provided credentials",
        )
        .into());
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
                    KeyCode::Enter => match get_token(&app.username, &app.password) {
                        Ok(token) => app.error_msg = token,
                        Err(e) => app.error_msg = e.to_string(),
                    },
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
        .margin(2)
        .constraints([Constraint::Max(3), Constraint::Max(3)].as_ref())
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
