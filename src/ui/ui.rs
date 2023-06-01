use std::{
    io,
    ops::Index,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use riven::models::match_v5::Participant;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::{
    display::{border_color, MatchDisplay, VecLine },
    no_data,
};

use super::{
    app::{App, Games, Msg},
    keys::handle_keys,
};

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
        //let name: Option<&str> = std::option_env!("WATCHER_NAME");
        //let region: Option<&str> = std::option_env!("WATCHER_REGION");
        //if let Some(name) = name {
        //    if let Some(region) = region {
        //        app.routes.get_item(Some(region.to_string()));
        //        msg = Some(Msg::Search(PlatformRoute::KR, name.to_string()));
        //    }
        //}

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
        None => no_data!(),
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(border_color(super::app::Window::Header, app.focus)),
    );
    f.render_widget(paragraph, chunks[0]);
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);

        let text = app.clone().input.get();
        let paragraph = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .style(border_color(super::app::Window::Input, app.focus)),
        );
        f.render_widget(paragraph, chunks[0]);

        let routes = Line::from(app.routes.print());
        let paragraph = Paragraph::new(routes).block(
            Block::default()
                .borders(Borders::ALL)
                .style(border_color(super::app::Window::Route, app.focus)),
        );
        f.render_widget(paragraph, chunks[1]);
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
        None => no_data!(),
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(border_color(super::app::Window::Rank, app.focus)),
    );
    f.render_widget(paragraph, area);
}

fn draw_masteries<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = match &app.data.masteries {
        Some(e) => {
            let a = e.iter().map(|f| f.spans()).collect::<Vec<_>>().concat();
            a
        }
        None => no_data!(),
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(border_color(super::app::Window::Masteries, app.focus)),
    );
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
    let mut curr_game: Vec<Line> = no_data!();
    let selected: MatchDisplay;

    match app.data.games.clone() {
        Games::G(g) => {
            state = g.state;

            for (_, game) in g.items.clone().into_iter().enumerate() {
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
                        text =
                            Span::styled("win", Style::default().fg(ratatui::style::Color::Green))
                    } else {
                        text = Span::styled("lose", Style::default().fg(ratatui::style::Color::Red))
                    }
                }

                items.push(ListItem::new(text));
            }
            if g.items.len() != 0 {
                selected = g.items.index(state.selected().unwrap_or(0)).clone();
                curr_game = selected.spans();
            }
        }
        Games::N => items.append(&mut vec![ListItem::new(no_data!())]),
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(ratatui::style::Color::Gray))
        .highlight_style(
            Style::default()
                .fg(ratatui::style::Color::LightCyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("=>");
    f.render_stateful_widget(list, chunks[0], &mut state);

    let paragraph = Paragraph::new(curr_game).block(
        Block::default()
            .borders(Borders::ALL)
            .style(border_color(super::app::Window::Games, app.focus)),
    );
    f.render_widget(paragraph, chunks[1]);
}

fn draw_footer<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let paragraph = Paragraph::new(format!("{:?}", app.route)).block(
        Block::default()
            .borders(Borders::ALL)
            .style(border_color(super::app::Window::Footer, app.focus)),
    );
    f.render_widget(paragraph, area);
}
