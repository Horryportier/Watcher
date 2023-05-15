use std::{
    collections::HashMap,
    io,
    ops::Index,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use riven::{consts::PlatformRoute, models::match_v5::Participant};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::utils::{MatchDisplay, VecSpans};

use super::app::{App, Games, Msg, Window};

pub async fn ui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let last_tick = Instant::now();
    let mut app = App::default();

    loop {
        let mut msg: Option<Msg> = None;
        terminal.draw(|f| msg = draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        match handle_keys(timeout, &mut app).await {
            Ok(x) => match x {
                Some(m) => match m {
                    Msg::Quit => {
                        thread::sleep(Duration::from_millis(500));

                        disable_raw_mode()?;

                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        return Ok(());
                    }
                    _ => msg = Some(m),
                },
                _ => {}
            },
            Err(_) => {}
        }
        app.msg = msg;
        app.msg().await;

        if last_tick.elapsed() >= tick_rate {}
    }
}

fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) -> Option<Msg> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_header(f, app, chunks[0]);
    draw_conntent(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
    None
}

fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);
    let text = match &app.data.summoner {
        Some(e) => e.spans(),
        None => vec![Spans::from("No data".to_string())],
    };

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, chunks[0]);
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);

        let text = app.clone().input.get();

        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
        f.render_widget(paragraph, chunks[0]);

        let items: Vec<ListItem> = app
            .route_map
            .items
            .iter()
            .map(|f| ListItem::new(f.0.as_str()))
            .collect();
        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(tui::style::Color::Gray))
            .highlight_style(
                Style::default()
                    .fg(tui::style::Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("=>");

        f.render_stateful_widget(list, chunks[1], &mut app.route_map.state);
    }
}

fn draw_conntent<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);

    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    draw_rank(f, app, chunk[0]);
    draw_masteries(f, app, chunk[1]);
    draw_games(f, app, chunks[1])
}

fn draw_rank<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = match &app.data.rank {
        Some(e) => {
            let a = e.iter().map(|f| f.spans()).collect::<Vec<_>>().concat();
            a
        }
        None => vec![Spans::from("No data".to_string())],
    };

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn draw_masteries<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = match &app.data.masteries {
        Some(e) => {
            let a = e.iter().map(|f| f.spans()).collect::<Vec<_>>().concat();
            a
        }
        None => vec![Spans::from("No data".to_string())],
    };

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn draw_games<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);

    let mut name: String = "".to_string();
    match &app.data.current_search {
        Some(i) => name = i.1.clone(),
        None => name = "".to_string().clone(),
    };

    let mut items: Vec<ListItem> = vec![];
    let mut state = ListState::default();
    let mut curr_game: Vec<Spans> = vec![Spans::from("no data")];
    let mut selected: MatchDisplay;
    match app.data.games.clone() {
        Games::G(g) => {
            state = g.state;
            for (i, game) in g.items.clone().into_iter().enumerate() {
                let mut text: Span = Span::from("");
                let mut id: Vec<&Participant> = game
                    .0
                    .info
                    .participants
                    .iter()
                    .filter(|f| f.summoner_name == name)
                    .collect();
                if id.len() != 0 {
                    if id.pop().unwrap().win {
                        text = Span::styled("win", Style::default().fg(tui::style::Color::Green))
                    } else {
                        text = Span::styled("lose", Style::default().fg(tui::style::Color::Red))
                    }
                }
                items.push(ListItem::new(text));
            }
            selected = g.items.index(state.selected().unwrap_or(0)).clone();
            curr_game = selected.spans();
        }
        Games::N(n) => items.append(&mut vec![ListItem::new(n)]),
    };
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(tui::style::Color::Gray))
        .highlight_style(
            Style::default()
                .fg(tui::style::Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("=>");
    f.render_stateful_widget(list, chunks[0], &mut state);

    let paragraph = Paragraph::new(curr_game).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, chunks[1]);
}

fn draw_footer<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let paragraph =
        Paragraph::new(format!("{:?}", app.focus)).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

async fn handle_keys(timeout: Duration, app: &mut App) -> io::Result<Option<Msg>> {
    if crossterm::event::poll(timeout)? {
        if app.focus.unwrap_or(super::app::Window::Header) != Window::Input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(Some(Msg::Quit)),
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
                    //KeyCode::Esc => app.pans/fn
                    //ipub tems.unselect(),
                    //KeyCode::Enter => match app.items.get_item() {
                    //    None => panic!("no item"),
                    //    Some(t) => match change(t.as_str()) {
                    //        Err(e) => panic!("{}", e),
                    //        Ok(_) => {}
                    //    },
                    //},
                    _ => {}
                }
            }
        }
        if app.focus.unwrap_or(Window::Input) == Window::Input {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Tab => return Ok(Some(Msg::Focus(Window::List))),
                    KeyCode::Enter => {
                        let tmp = app.clone().input.get();
                        app.input.clear();
                        return Ok(Some(Msg::Search(riven::consts::PlatformRoute::EUN1, tmp)));
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
