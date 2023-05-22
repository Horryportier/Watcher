use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use super::app::{App, Msg, Window};

pub async fn handle_keys(timeout: Duration, app: &mut App) -> io::Result<Option<Msg>> {
    if crossterm::event::poll(timeout)? {
        if app.focus.unwrap_or(super::app::Window::Header) != Window::Input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(Some(Msg::Quit)),
                    KeyCode::Esc => return Ok(Some(Msg::Quit)),
                    KeyCode::Char('f') => {
                        return Ok(Some(Msg::Search(
                            riven::consts::PlatformRoute::EUN1,
                            "NOTJOHNYS".to_string(),
                        )))
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
                        app.input.clear();
                        return Ok(Some(Msg::Search(app.route, tmp)));
                    }
                    KeyCode::Char(c) => app.input.append(c.to_string()),
                    KeyCode::Backspace => app.input.delete(),
                    _ => {}
                }
            }
        }
    }
    Ok(None)
}
