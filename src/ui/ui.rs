use std::{
    io, thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal, style::Style,
};

use super::app::{App, Msg};

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

    let text = match &app.data.summoner {
        Some(e) => e.spans(),
        None => vec![Spans::from("No data".to_string())],
    };

    let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
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
    let paragraph = Paragraph::new("games").block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn draw_footer<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let paragraph = Paragraph::new("footer").block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

async fn handle_keys(timeout: Duration, app: &mut App) -> io::Result<Option<Msg>> {
    if crossterm::event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(Some(Msg::Quit)),
                KeyCode::Char('f') => {
                    return Ok(Some(Msg::Search(
                        riven::consts::PlatformRoute::EUN1,
                        "HorryPortier6".to_string(),
                    )))
                }
                KeyCode::Tab => app.focus = Some(app.focus.unwrap_or(super::app::Window::Header).next()),
                //KeyCode::Char('j') => app.items.next(),
                //KeyCode::Down => app.items.next(),
                //KeyCode::Up => app.items.previous(),
                //KeyCode::Char('k') => app.items.previous(),
                //KeyCode::Esc => app.items.unselect(),
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
    Ok(None)
}
