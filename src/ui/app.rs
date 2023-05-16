use std::collections::HashMap;

use riven::consts::PlatformRoute;
use tui::{
    style::Style,
    text::{Span, Spans},
    widgets::ListState,
};

use crate::{
    api::api::{get_games, get_masteries, get_rank, get_summoner},
    utils::{ChampionMasteryDisplay, LeagueEntryDisplay, MatchDisplay, SummonerDisplay, With},
};

#[derive(Debug, Clone)]
pub enum State {
    Searching(String, PlatformRoute), // name PlatformRoute
    Failed(String, PlatformRoute),
    Idle,
}

#[derive(Debug, Clone)]
pub enum Msg {
    Quit,
    Focus(Window),
    Input(Window),
    Search(PlatformRoute, String),
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
        let w = vec![
            Window::Header,
            Window::Input,
            Window::Rank,
            Window::Masteries,
            Window::List,
            Window::Games,
            Window::Footer,
        ];
        let i = w.iter().position(|f| f == self).unwrap_or(0);
        if i >= w.len() - 1 {
            w[0]
        } else {
            w[i + 1]
        }
    }
}

#[derive(Clone)]
pub struct App {
    pub state: State,
    pub msg: Option<Msg>,
    pub focus: Option<Window>,
    pub data: Data,
    pub input: Input,
    pub route: PlatformRoute,
    pub route_map: RouteList,
}

#[derive(Clone)]
pub struct Data {
    pub rank: Option<Vec<LeagueEntryDisplay>>,
    pub current_search: Option<(String, String)>, // (id,name)
    pub masteries: Option<Vec<ChampionMasteryDisplay>>,
    pub summoner: Option<SummonerDisplay>,
    pub games: Games,
}

#[derive(Clone)]
pub struct CurrentSearch(pub String, pub String);

#[derive(Clone)]
pub enum Games {
    N(String),
    G(GamesList),
}

impl App {
    pub fn default() -> App {
        let mut map: HashMap<String, PlatformRoute> = HashMap::new();
        map.insert("kr".to_string(), PlatformRoute::KR);
        map.insert("ru".to_string(), PlatformRoute::RU);
        map.insert("br".to_string(), PlatformRoute::BR1);
        map.insert("jp".to_string(), PlatformRoute::JP1);
        map.insert("la1".to_string(), PlatformRoute::LA1);
        map.insert("la2".to_string(), PlatformRoute::LA2);
        map.insert("na".to_string(), PlatformRoute::NA1);
        map.insert("oce".to_string(), PlatformRoute::OC1);
        map.insert("ph".to_string(), PlatformRoute::PH2);
        map.insert("sg".to_string(), PlatformRoute::SG2);
        map.insert("th".to_string(), PlatformRoute::TH2);
        map.insert("tr".to_string(), PlatformRoute::TR1);
        map.insert("tw".to_string(), PlatformRoute::TW2);
        map.insert("eune".to_string(), PlatformRoute::EUN1);
        map.insert("euw".to_string(), PlatformRoute::EUW1);

        let route_map = RouteList {
            state: ListState::default(),
            items: map,
        };
        App {
            state: State::Idle,
            msg: None,
            focus: Some(Window::List),
            input: Input {
                content: "".to_string(),
            },
            route: PlatformRoute::KR,
            route_map,
            data: Data {
                rank: None,
                current_search: None,
                masteries: None,
                summoner: None,
                games: Games::N("NO data".to_string()),
            },
        }
    }

    pub async fn msg(&mut self) {
        let msg = &self.msg;

        match msg {
            Some(msg) => match msg {
                Msg::Quit => {}
                Msg::Focus(w) => {
                    self.focus = Some(*w);
                    self.msg = None
                }
                Msg::Search(route, name) => {
                    let puuid: &str;
                    let res = get_summoner(*route, &name).await.unwrap_or(None);
                    match res {
                        None => self.state = State::Failed(name.to_string(), *route),

                        Some(sumoner) => {
                            self.state = State::Searching(name.to_string(), *route);
                            puuid = &sumoner.puuid;
                            self.data.summoner = Some(SummonerDisplay::with(sumoner.clone()));
                            self.data.current_search = Some((sumoner.id, sumoner.name));

                            let res =
                                get_rank(*route, &self.data.current_search.as_ref().unwrap().0)
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

                            let res = get_games(*route, puuid).await;
                            let entry: Option<Vec<MatchDisplay>> = match res {
                                Err(_) => None,
                                Ok(rank) => Some(
                                    rank.iter().map(|f| MatchDisplay::with(f.clone())).collect(),
                                ),
                            };
                            match entry {
                                Some(e) => self.data.games = Games::G(GamesList::with(e)),
                                None => {
                                    self.data.games =
                                        Games::N("couldn't get games data".to_string())
                                }
                            }

                            self.msg = None;

                            self.focus = Some(Window::List);
                            self.state = State::Idle
                        }
                    }
                }
                //Msg::Input(w) => {
                //    self.focus = Some(*w);
                //    match self.focus.unwrap_or(Window::Header) {
                //        Window::Input => {
                //            self.input.state = true;
                //        }
                //        _ => {}
                //    }
                //}
                _ => {}
            },
            None => {}
        }
    }

    pub fn up(&mut self) {
        match self.focus.unwrap_or(Window::Header) {
            Window::List => match self.data.games {
                Games::G(ref mut g) => g.previous(),
                Games::N(_) => {}
            },
            Window::Route => {
                self.route_map.previous();
                self.route = self.route_map.get_item()
            }
            _ => {}
        }
    }
    pub fn down(&mut self) {
        match self.focus.unwrap_or(Window::Header) {
            Window::List => match self.data.games {
                Games::G(ref mut g) => g.next(),
                Games::N(_) => {}
            },
            Window::Route => {
                self.route_map.next();
                self.route = self.route_map.get_item()
            }
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct GamesList {
    pub state: ListState,
    pub items: Vec<MatchDisplay>,
}

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
    pub items: HashMap<String, PlatformRoute>,
}

impl RouteList {
    pub fn with(items: HashMap<String, PlatformRoute>) -> RouteList {
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

        for (i, s) in self.items.keys().into_iter().enumerate() {
            if self.state.selected().unwrap_or(0) == i {
                v.append(
                    &mut [
                        Span::styled(s, Style::default().fg(tui::style::Color::Cyan)),
                        Span::from(" | "),
                    ]
                    .to_vec(),
                );
                continue;
            }
            v.append(
                &mut [
                    Span::styled(s, Style::default().fg(tui::style::Color::Red)),
                    Span::from(" | "),
                ]
                .to_vec(),
            );
        }
        v
    }

    pub fn get_item(&mut self) -> PlatformRoute {
        for (i, s) in self.items.keys().into_iter().enumerate() {
            if self.state.selected().unwrap_or(0) == i {
                let (_, route) = self
                    .items
                    .get_key_value(s)
                    .unwrap_or((&"kr".to_string(), &PlatformRoute::KR));
                return *route;
            }
        }
        PlatformRoute::KR
    }
}
