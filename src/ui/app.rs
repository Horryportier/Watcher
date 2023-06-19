use std::fmt::Error;

use ratatui::{style::Style, text::Span, widgets::ListState};
use riven::consts::PlatformRoute;

use crate::{
    api::api::{get_games, get_masteries, get_rank, get_summoner},
    display::{ChampionMasteryDisplay, LeagueEntryDisplay, MatchDisplay, SummonerDisplay, With}, utils::{Log, routes, parse_route},
};

use super::keys::Keys;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum State {
    Searching(String, PlatformRoute), // name PlatformRoute
    Failed(String, PlatformRoute),
    Error(WatcherErr),
    Idle,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum WatcherErr {
    SearchFalied { name: String, puuid: String },
    Riot(String),
}

#[derive(Debug, Clone)]
pub enum Msg {
    Quit,
    Focus(Window),
    Search(PlatformRoute, String),
    None,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Window {
    Header,
    Input,
    Route,
    Rank,
    Masteries,
    List,
    Games,
    Footer,
}

impl Window {
    pub fn next(&self) -> Window {
        let windows = vec![
            Window::Header,
            Window::Input,
            Window::Route,
            Window::Rank,
            Window::Masteries,
            Window::List,
            Window::Games,
            Window::Footer,
        ];
        for (i, w) in windows.iter().enumerate() {
            if self == w {
                if i + 1 >= windows.len() {
                    return windows[0];
                }
                return windows[i + 1];
            }
        }
        Window::Header
    }
}

#[derive(Clone)]
pub struct App {
    pub api_key: String, 
    pub state: State,
    pub msg: Option<Msg>,
    pub focus: Option<Window>,
    pub data: Data,
    pub input: Input,
    pub route: PlatformRoute,
    pub routes: RouteList,
    pub keys: Keys,
    pub log: Log,
}

#[derive(Clone)]
pub struct Data {
    pub rank: Option<Vec<LeagueEntryDisplay>>,
    pub current_search: Option<(String, String)>, // (id,name)
    pub env_search: Option<(String, String)>,
    pub masteries: Option<Vec<ChampionMasteryDisplay>>,
    pub summoner: Option<SummonerDisplay>,
    pub games: Games,
}

#[derive(Clone)]
pub struct CurrentSearch(pub String, pub String);

#[derive(Clone)]
pub enum Games {
    N,
    G(GamesList),
}

impl App {
    pub fn default() -> App {
        let map = routes();
        let routes = RouteList {
            state: ListState::default(),
            items: map,
        };

        let keys = Keys::default();

        App {
            keys,
            log: Log::new(crate::utils::LogKind::Info, "App start".into()), 
            api_key: "".to_string(),// api key
            state: State::Idle,
            msg: None,
            focus: Some(Window::List),
            input: Input {
                content: "".to_string(),
            },
            route: PlatformRoute::KR,
            routes,
            data: Data {
                rank: None,
                current_search: None,
                env_search: None,
                masteries: None,
                summoner: None,
                games: Games::N,
            },
        }
    }

    pub async fn msg(&mut self) {
        let msg = self.msg.clone();

        match msg {
            Some(msg) => match msg {
                Msg::Quit => {}
                Msg::Focus(w) => {
                    self.focus = Some(w);
                    self.msg = None
                }
                Msg::Search(route, name) => {
                    self.input.clear();
                    self.search_all(&route, &name).await;
                    self.log = Log::new(crate::utils::LogKind::Info, "search finished".into())
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn get_env_search(&mut self) -> Result<(), Error> {
        let name: Option<&str> = std::option_env!("WATCHER_NAME");
        let region: Option<&str> = std::option_env!("WATCHER_REGION");
        if let Some(name) = name {
            if let Some(region) = region {
                self.data.env_search = Some((name.into(), region.into()));
                return Ok(());
            }
            return Err(Error::default());
        }
        Err(Error::default())
    }

    pub fn up(&mut self) {
        match self.focus.unwrap_or(Window::Header) {
            Window::List => match self.data.games {
                Games::G(ref mut g) => g.previous(),
                Games::N => {}
            },
            Window::Route => {
                self.routes.previous();
                self.route = self.routes.get_item(None)
            }
            _ => {}
        }
    }
    pub fn down(&mut self) {
        match self.focus.unwrap_or(Window::Header) {
            Window::List => match self.data.games {
                Games::G(ref mut g) => g.next(),
                Games::N => {}
            },
            Window::Route => {
                self.routes.next();
                self.route = self.routes.get_item(None)
            }
            _ => {}
        }
    }

    pub fn enter(&mut self) -> Msg {
        match self.focus.unwrap_or(Window::Header) {
            Window::Route => Msg::Search(self.route, self.input.clone().get()),
            _ => Msg::None,
        }
    }

    pub fn into_route(mut self, text: String) -> PlatformRoute {
        self.routes.get_item(Some(text))
    }

    async fn search_all(&mut self, route: &PlatformRoute, name: &str) {
        let name = name.replace(" ", "");
        let puuid: &str;
        let res = get_summoner(&self.api_key, *route, &name).await.unwrap_or(None);
        match res {
            None => self.state = State::Failed(name.to_string(), *route),

            Some(sumoner) => {
                self.state = State::Searching(name.to_string(), *route);
                puuid = &sumoner.puuid;
                self.data.summoner = Some(SummonerDisplay::with(sumoner.clone()));
                self.data.current_search = Some((sumoner.id, sumoner.name));

                let res = get_rank(
                    &self.api_key,
                    *route,
                    &self.data.current_search.as_ref().unwrap().0,
                )
                .await;
                let entry: Option<Vec<LeagueEntryDisplay>> = match res {
                    Err(_) => None,
                    Ok(rank) => Some(
                        rank.iter()
                            .map(|f| LeagueEntryDisplay::with(f.clone()))
                            .collect(),
                    ),
                };
                self.data.rank = entry;

                let res = get_masteries(
                    &self.api_key,
                    *route,
                    &self.data.current_search.as_ref().unwrap().0,
                    10,
                )
                .await;
                let entry: Option<Vec<ChampionMasteryDisplay>> = match res {
                    Err(_) => None,
                    Ok(m) => Some(
                        m.iter()
                            .map(|f| ChampionMasteryDisplay::with(f.clone()))
                            .collect(),
                    ),
                };
                self.data.masteries = entry;

                let res = get_games(&self.api_key, *route, puuid).await;
                let entry: Option<Vec<MatchDisplay>> = match res {
                    Err(_) => None,
                    Ok(rank) => Some(rank.iter().map(|f| MatchDisplay::with(f.clone())).collect()),
                };
                match entry {
                    Some(e) => self.data.games = Games::G(GamesList::with(e)),
                    None => self.data.games = Games::N,
                }

                self.msg = None;

                self.focus = Some(Window::List);
                self.state = State::Idle
            }
        }
    }
}

#[derive(Clone)]
pub struct GamesList {
    pub state: ListState,
    pub items: Vec<MatchDisplay>,
}


#[allow(dead_code)]
impl GamesList {
    pub fn with(items: Vec<MatchDisplay>) -> GamesList {
        GamesList {
            state: ListState::default(),
            items,
        }
    }

    pub fn default(items: Vec<MatchDisplay>) -> GamesList {
        GamesList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i <= 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.state.select(Some(i));
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn get_item(&mut self) -> Option<&MatchDisplay> {
        match self.state.selected() {
            None => return None,
            Some(i) => return Some(&self.items[i]),
        };
    }
}

#[derive(Clone)]
pub struct Input {
    pub content: String,
}

impl Input {
    pub fn get(self) -> String {
        self.content.clone()
    }

    pub fn append(&mut self, rhs: String) {
        self.content = format!("{}{}", self.content, rhs)
    }
    pub fn set(&mut self, text: String) {
        self.content = text
    }

    pub fn clear(&mut self) {
        self.content = "".to_string()
    }

    pub fn delete(&mut self) {
        if self.content.len() != 0 {
            let _ = self.content.remove(self.content.len() - 1);
        }
    }
}

#[derive(Clone)]
pub struct RouteList {
    pub state: ListState,
    pub items: Vec<(String, PlatformRoute)>,
}

#[allow(dead_code)]
impl RouteList {
    pub fn with(items: Vec<(String, PlatformRoute)>) -> RouteList {
        RouteList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn print(&mut self) -> Vec<Span> {
        let mut v: Vec<Span> = vec![];

        for (i, s) in self.items.iter().enumerate() {
            if self.state.selected().unwrap_or(0) == i {
                v.append(
                    &mut [
                        Span::styled(
                            s.clone().0,
                            Style::default().fg(ratatui::style::Color::Cyan),
                        ),
                        Span::from(" | "),
                    ]
                    .to_vec(),
                );
                continue;
            }
            v.append(
                &mut [
                    Span::styled(s.clone().0, Style::default().fg(ratatui::style::Color::Red)),
                    Span::from(" | "),
                ]
                .to_vec(),
            );
        }
        v
    }

    pub fn get_item(&mut self, key: Option<String>) -> PlatformRoute {
        match key {
            Some(key) =>  parse_route(key),
                None => {
                let idx = self.state.selected().unwrap_or(0);

                if idx >= self.items.len() {
                    return PlatformRoute::KR;
                }
                self.items[idx].1
            }
        }
    }
}
