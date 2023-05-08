use std::ops::Index;

use riven::consts::PlatformRoute;

use crate::{
    api::api::{get_masteries, get_rank, get_summoner},
    utils::{ChampionMasteryDisplay, LeagueEntryDisplay, SummonerDisplay},
};

#[derive(Debug)]
pub enum Msg {
    Quit,
    Focus(Window),
    Search(PlatformRoute, String),
}

#[derive( PartialEq, Eq)] 
#[derive(Debug, Clone, Copy)]
pub enum Window {
    Header,
    Input,
    Rank,
    Masteries,
    Games,
    Footer,
}

impl Window {
     pub fn next(&self)-> Window  {
        let w = vec![
            Window::Header,
            Window::Input,
            Window::Rank,
            Window::Masteries,
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

pub struct App {
    pub msg: Option<Msg>,
    pub focus: Option<Window>,
    pub data: Data,
}

pub struct Data {
    pub rank: Option<Vec<LeagueEntryDisplay>>,
    pub current_search: Option<(String, String)>, // (id,name)
    pub masteries: Option<Vec<ChampionMasteryDisplay>>,
    pub summoner: Option<SummonerDisplay>,
}

impl App {
    pub fn default() -> App {
        App {
            msg: None,
            focus: None,
            data: Data {
                rank: None,
                current_search: None,
                masteries: None,
                summoner: None,
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
                    let res = get_summoner(*route, &name).await.unwrap_or(None);
                    match res {
                        None => {}

                        Some(sumoner) => {
                            self.data.summoner = Some(SummonerDisplay::with(sumoner.clone()));
                            self.data.current_search = Some((sumoner.id, sumoner.name));

                            let res = get_rank(
                                *route,
                                self.data.current_search.as_ref().unwrap().0.as_str(),
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
                                self.data.current_search.as_ref().unwrap().0.as_str(),
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

                            self.msg = None
                        }
                    }
                }
            },
            None => {}
        }
    }
}
