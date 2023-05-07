use riven::consts::PlatformRoute;

use crate::{
    api::api::{get_rank, get_summoner},
    utils::LeagueEntryDisplay,
};

#[derive(Debug)]
pub enum Msg {
    Quit,
    Focus(Window),
    Search(PlatformRoute, String),
}

#[derive(Debug, Clone, Copy)]
pub enum Window {
    Header,
    Input,
    Rank,
    Masteries,
    Games,
    Footer,
}

pub struct App {
    pub msg: Option<Msg>,
    pub focus: Option<Window>,
    pub data: Data,
}

pub struct Data {
    pub rank: Option<Vec<LeagueEntryDisplay>>,
    pub current_search: Option<(String, String)>, // (id,name)
}

impl App {
    pub fn default() -> App {
        App {
            msg: None,
            focus: None,
            data: Data {
                rank: None,
                current_search: None,
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
                            self.msg = None
                        }
                    }
                }
            },
            None => {}
        }
    }
}
