use std::{fmt::Display, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    style::Stylize,
};

use crate::utils::Log;

use super::app::{App, Msg, Window};

#[derive(Clone)]
pub struct Keys {
    pub keys: Vec<(Vec<KeyCode>, String)>,
}

impl Keys {
    #[allow(dead_code)]
    pub fn with(keys: Vec<(Vec<KeyCode>, String)>) -> Keys {
        Keys { keys }
    }
}

impl Default for Keys {
    fn default() -> Self {
        let keys: Vec<(Vec<KeyCode>, String)> = vec![
            (vec![KeyCode::Char('q'), KeyCode::Esc], "Quit".into()),
            (vec![KeyCode::Char('i')], "focus input".into()),
            (vec![KeyCode::Tab], "switch window".into()),
            (vec![KeyCode::Down, KeyCode::Char('j')], "down".into()),
            (vec![KeyCode::Up, KeyCode::Char('j')], "up".into()),
            (vec![KeyCode::Enter], "search".into()),
            (vec![KeyCode::Insert], "clipboard".into()),
            (vec![KeyCode::Delete], "clear input".into()),
            (vec![KeyCode::Char('f')], "search with ENV vars".into()),
        ];
        Keys { keys }
    }
}

impl Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.keys.iter().map(|f| {
            let k =
                f.0.iter()
                    .map(|f| format!("{:?}", {
                        match  f {
                            KeyCode::Char(c) => format!("{c}"),
                            _ => format!("{:?}", f)
                        }}).with(crossterm::style::Color::Green).to_string())
                    .collect::<Vec<String>>()
                    .join(&"/".with(crossterm::style::Color::Reset).to_string());
           format!("{} -> {}", k, f.1.clone().with(crossterm::style::Color::Yellow).to_string()) 
        }).collect::<Vec<String>>().join(" | ");
        write!(f, "{}", text)
    }
}

pub async fn handle_keys(timeout: Duration, app: &mut App) -> io::Result<Option<Msg>> {
    if crossterm::event::poll(timeout)? {
        if app.focus.unwrap_or(super::app::Window::Header) != Window::Input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(Some(Msg::Quit)),
                    KeyCode::Esc => return Ok(Some(Msg::Quit)),
                    KeyCode::Enter => return Ok(Some(app.enter())),
                    KeyCode::Char('f') => {
                        let msg = match app.data.clone().env_search {
                            None => match app.get_env_search() {
                                Ok(..) => None,
                                Err(..) => None,
                            },
                            Some(search) => {
                                app.log = Log::new(crate::utils::LogKind::Info, 
                                        format!("searching: {} {}", app.clone().into_route(search.1.clone()), search.0.clone()));
                                Some(Msg::Search(app.clone().into_route(search.1), search.0))
                            }
                        };
                        return Ok(msg);
                    }
                    KeyCode::Tab => {
                        app.focus = Some(app.focus.unwrap_or(super::app::Window::Header).next())
                    }
                    KeyCode::Char('j') => app.down(),
                    KeyCode::Down => app.down(),
                    KeyCode::Up => app.up(),
                    KeyCode::Char('k') => app.up(),
                    KeyCode::Char('i') => return Ok(Some(Msg::Focus(super::app::Window::Input))),
                    KeyCode::Char('r') => return Ok(Some(Msg::Focus(super::app::Window::Route))),
                    KeyCode::Char('l') => return Ok(Some(Msg::Focus(super::app::Window::List))),
                    _ => {}
                }
            }
        }
        if app.focus.unwrap_or(Window::Input) == Window::Input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(Some(Msg::Quit)),
                    KeyCode::Tab => {
                        app.focus = Some(app.focus.unwrap_or(super::app::Window::Header).next())
                    }
                    KeyCode::Enter => {
                        let tmp = app.clone().input.get();
                                app.log = Log::new(crate::utils::LogKind::Info, 
                                        format!("searching: {} {}", app.route, tmp));
                        return Ok(Some(Msg::Search(app.route, tmp)));
                    }
                    KeyCode::Char(c) => app.input.append(c.to_string()),
                    KeyCode::Backspace => app.input.delete(),
                    KeyCode::Insert => {
                        let clip = cli_clipboard::get_contents().unwrap_or("".to_string());
                        app.input.set(clip)
                    }
                    KeyCode::Delete => app.input.clear(),
                    _ => {}
                }
            }
        }
    }
    Ok(None)
}
