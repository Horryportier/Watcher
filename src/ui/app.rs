use riven::consts::PlatformRoute;
use tui::widgets::{ListItem, ListState};

use crate::{
    api::api::{get_games, get_masteries, get_rank, get_summoner},
    utils::{ChampionMasteryDisplay, LeagueEntryDisplay, MatchDisplay, SummonerDisplay},
};

#[derive(Debug, Clone)]
pub enum Msg {
    Quit,
    Focus(Window),
    Search(PlatformRoute, String),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Window {
    Header,
    Input,
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
pub struct App<'a> {
    pub msg: Option<Msg>,
    pub focus: Option<Window>,
    pub data: Data<'a>,
}

#[derive(Clone)]
pub struct Data<'a> {
    pub rank: Option<Vec<LeagueEntryDisplay>>,
    pub current_search: Option<CurrentSearch<'a>>, // (id,name)
    pub masteries: Option<Vec<ChampionMasteryDisplay>>,
    pub summoner: Option<SummonerDisplay>,
    pub games: Games,
}

#[derive(Clone)]
pub struct CurrentSearch<'a>(pub &'a str, pub &'a str);
    

#[derive(Clone)]
pub enum Games {
    N(String),
    G(GamesList<MatchDisplay>),
}

impl<'a> App<'a> {
    pub fn default() -> App<'a> {
        App {
            msg: None,
            focus: None,
            data: Data {
                rank: None,
                current_search: None,
                masteries: None,
                summoner: None,
                games: Games::N("NO datak".to_string()),
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
                        None => {}

                        Some(sumoner) => {
                            puuid = &sumoner.puuid;
                            self.data.summoner = Some(SummonerDisplay::with(sumoner.clone()));
                            self.data.current_search =
                                Some(CurrentSearch(&sumoner.id, &sumoner.name));

                            let res = get_rank(
                                *route,
                                self.data.current_search.as_ref().unwrap().0
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
                                *route,
                                self.data.current_search.as_ref().unwrap().0,
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
                            self.data.games = Games::G(GamesList::with(entry.unwrap()));

                            self.msg = None
                        }
                    }
                }
            },
            None => {}
        }
    }

    pub fn up(&mut self) {
        match self.focus.unwrap_or(Window::Header) {
            Window::List => match self.data.games {
                Games::G(mut g) => g.previous(),
                Games::N(_) => {}
            },
            _ => {}
        }
    }
    pub fn down(&mut self) {
        match self.focus.unwrap_or(Window::Header) {
            Window::List => match self.data.games {
                Games::G(mut g) => g.next(),
                Games::N(_) => {}
            },
            _ => {}
        }
    }
}

#[derive(Clone)]
pub struct GamesList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> GamesList<T> {
    pub fn with(items: Vec<T>) -> GamesList<T> {
        GamesList {
            state: ListState::default(),
            items,
        }
    }

    pub fn default(items: Vec<T>) -> GamesList<T> {
        GamesList {
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

    pub fn get_item(&mut self) -> Option<&T> {
        match self.state.selected() {
            None => return None,
            Some(i) => return Some(&self.items[i]),
        };
    }
}
