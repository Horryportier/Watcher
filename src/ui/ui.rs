use std::{
    io,
    ops::Index,
    thread,
    time::{Duration, Instant},
};

use ansi_to_tui::IntoText;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use riven::models::match_v5::Participant;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{
    display::{border_color, concat_text, DisplayToText, MatchDisplay},
    no_data,
};

use super::{
    app::{App, Games, Msg},
    keys::handle_keys,
};

pub async fn ui(api_key: &str) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let last_tick = Instant::now();
    let mut app = App::default();
    app.api_key = api_key.to_string();

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
        let _ = app.get_env_search();

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

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ]
            .as_ref(),
        )
        .split(chunks[2]);


    draw_footer(f, app, chunks[0]);
    draw_logs(f, app, chunks[1]);
    None
}

fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);
    let text = match &app.data.summoner {
        Some(e) => e.clone().into_text(),
        None => no_data!(),
    };

    let paragraph =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).style(border_color(
            super::app::Window::Header,
            app.focus,
            None,
        )));
    f.render_widget(paragraph, chunks[0]);
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(chunks[1]);

        let mut text = Text::styled(app.clone().input.get(), Style::default().fg(Color::Green));
        if app.input.content == "" {
            text = Text::styled("Input", Style::default().fg(Color::Yellow))
        }
        
        
        let paragraph =
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).style(border_color(
                super::app::Window::Input,
                app.focus,
                Some((Color::Green, Color::White)),
            )));
        f.render_widget(paragraph, chunks[0]);

        let routes = Line::from(app.routes.print());
        let paragraph = Paragraph::new(routes).block(
            Block::default().borders(Borders::ALL).style(border_color(
                super::app::Window::Route,
                app.focus,
                None,
            )),
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
    let texts: Vec<Text> = match &app.data.rank {
        Some(e) => e.iter().map(|f| f.into_text()).collect::<Vec<Text>>(),
        None => vec![no_data!()],
    };
    let paragraph = Paragraph::new(concat_text(texts)).block(
        Block::default().borders(Borders::ALL).style(border_color(
            super::app::Window::Rank,
            app.focus,
            None,
        )),
    );
    f.render_widget(paragraph, area);
}

fn draw_masteries<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let texts = match &app.data.masteries {
        Some(e) => e.iter().map(|f| f.into_text()).collect::<Vec<_>>(),
        None => vec![no_data!()],
    };
    let paragraph = Paragraph::new(concat_text(texts)).block(
        Block::default().borders(Borders::ALL).style(border_color(
            super::app::Window::Masteries,
            app.focus,
            None,
        )),
    );
    f.render_widget(paragraph, area);
}

fn draw_games<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(area);
                                                                                                                                                                    
    #[allow(unused_assignments)]
    let mut name: String = String::default();
    match &app.data.current_search {
        Some(i) => name = i.1.clone(),
        None => name = "".to_string().clone(),
    };

    let mut items: Vec<ListItem> = vec![];
    let mut state = ListState::default();
    let mut curr_game: Text = Text::from("");
    let selected: MatchDisplay;

    match app.data.games.clone() {
        Games::G(g) => {
            state = g.state;

            for (_, game) in g.items.clone().into_iter().enumerate() {
                let text: Text;
                let mut id: Vec<&Participant> = game
                    .0
                    .info
                    .participants
                    .iter()
                    .filter(|f| f.summoner_name == name)
                    .collect();

                if id.len() != 0 {
                    match id.pop().unwrap().win {
                        true => text = Text::styled("win", Style::default().fg(Color::Green)),
                        false => text = Text::styled("lose", Style::default().fg(Color::Red)),
                    }
                } else {
                     text = Text::styled("no_data", Style::default().fg(Color::Red))
                }

                items.push(ListItem::new(text));
            }
            if g.items.len() != 0 {
                selected = g.items.index(state.selected().unwrap_or(0)).clone();
                curr_game = selected.into_text();
            }
        }
        Games::N => items.append(&mut vec![ListItem::new(Text::from("no data"))]),
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
        Block::default().borders(Borders::ALL).style(border_color(
            super::app::Window::Games,
            app.focus,
            None,
        )),
    );
    f.render_widget(paragraph, chunks[1]);
}

fn draw_footer<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = format!("{}", app.keys);

    let paragraph = Paragraph::new(text.into_text().unwrap_or(no_data!()))
        .block(Block::default().borders(Borders::ALL).style(border_color(
            super::app::Window::Footer,
            app.focus,
            None,
        )))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_logs<B: Backend>(f: &mut Frame<B> , app: &mut App, area: Rect) {
    let text = app.log.to_string();
    let paragraph = Paragraph::new(text.into_text().unwrap_or(no_data!()))
        .block(Block::default().borders(Borders::ALL).style(border_color(
            super::app::Window::Footer,
            app.focus,
            None,
        )))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area)
}
