use crossterm::style::Stylize;
use riven::{consts::PlatformRoute, RiotApiError};

use crate::{utils::{ROUTE_NAMES, parse_route, print_help, is_numeric}, 
    display::{SummonerDisplay, With, LeagueEntryDisplay, ChampionMasteryDisplay, MatchDisplay}, 
    api::api::{get_summoner, get_rank, get_masteries, get_games}, };

const GET_SUMMONER_ERR: &str = "couldn't get_summoner";
const SUMMONER_IS_NONE: &str = "summoner is none";
const GET_RANK_ERR: &str = "couldn't get rank";
const GET_MASTERIES_ERR: &str = "couldn't get masteries";
const GET_GAMES_ERR: &str = "couldn't get games";

#[derive(Debug)]
#[allow(dead_code)]
pub enum Arg {
    Route(PlatformRoute),
    Indent(String),
    Int(usize),

    HelpFlag,

    SummonerFlag,
    RankFlag,
    MasteryFlag,
    GameFlag(usize)
    
}

#[derive(Debug)]
pub struct Args {
    pub args: Vec<Arg>,
}


impl Args {
    pub fn new(input: Vec<String>) -> Args{
        let args = Args::parse(input);
        Args { args }
    }

    fn parse(args: Vec<String>) -> Vec<Arg>{
        let mut a: Vec<Arg> = Vec::new();
        for arg in  args {
            a.push(match arg.as_str() {
                "-h" | "--help" => Arg::HelpFlag,
                "-s" | "--sum" => Arg::SummonerFlag,
                "-r" | "--rank" => Arg::RankFlag,
                "-m" | "-mastery" => Arg::MasteryFlag,
                "-g" | "-game" =>   Arg::GameFlag(arg.parse::<usize>().unwrap_or(0)),
                arg if is_numeric(arg) =>  Arg::Int(arg.parse::<usize>().unwrap_or(0)),
                arg if ROUTE_NAMES.contains(&arg) => Arg::Route(parse_route(arg.into())),
                _ =>  Arg::Indent(arg.into())
            });
        }    
        a
    }

    pub async fn  execute(&self, api_key: &str) -> Result<(), RiotApiError>{
        let route: Option<PlatformRoute> = self.args.iter().find_map(|f| match *f {
            Arg::Route(route) => Some(route),
            _ => None,
        }); 
        
        let mut names: Vec<String> = self.args.iter().map(|f| {
        match f {
           Arg::Indent(ident)  => ident.into(),
           _ => "".into(), 
        }}).collect::<Vec<String>>();
        names.retain(|f| f != "");
        
        println!("players [{:?}] {}",route, 
                 names.iter()
                 .map(|f| f.clone()
                      .with(crossterm::style::Color::Green)
                      .to_string()).collect::<Vec<String>>().join(" "));

        for arg in &self.args {
            match arg {
                Arg::HelpFlag => print_help(),
                Arg::SummonerFlag =>  print_summoner(api_key.into(),route.unwrap_or(PlatformRoute::KR), names.clone()).await,
                Arg::RankFlag => print_rank(api_key.into(),route.unwrap_or(PlatformRoute::KR), names.clone()).await,
                Arg::MasteryFlag => print_mastery(api_key.into(),route.unwrap_or(PlatformRoute::KR), names.clone()).await,
                Arg::GameFlag(game) => {
                            print_game(api_key.into(),route.unwrap_or(PlatformRoute::KR), names.clone(), *game).await
                },
                _ => {}
            }
        }

        Ok(())
    }
}


async fn print_summoner(api_key: String, route: PlatformRoute, names: Vec<String>) {
     for name in names{
                let sum = get_summoner(&api_key, route, &name)
                    .await
                    .expect(&GET_SUMMONER_ERR)
                    .expect(&SUMMONER_IS_NONE);
                println!("{}", SummonerDisplay::with(sum));
     }
}

async fn print_rank(api_key: String, route: PlatformRoute, names: Vec<String>) {
    for name in names {
                let id = get_summoner(&api_key, route, &name)
                    .await
                    .expect(&GET_SUMMONER_ERR)
                    .expect(&SUMMONER_IS_NONE)
                    .id;
                let res = get_rank(&api_key, route, id.as_str())
                    .await
                    .expect(GET_RANK_ERR);
                let ranks: Vec<LeagueEntryDisplay> = res
                    .iter()
                    .map(|f| LeagueEntryDisplay::with(f.clone()))
                    .collect();
                for r in ranks {
                    print!("{}", r);
                }
    }
}

async fn print_mastery(api_key: String, route: PlatformRoute, names: Vec<String>) {
    for name in names {
let id = get_summoner(&api_key, route, &name)
                    .await
                    .expect(&GET_SUMMONER_ERR)
                    .expect(&SUMMONER_IS_NONE)
                    .id;
                let masteries: Vec<ChampionMasteryDisplay> = get_masteries(&api_key, route, &id, 10)
                    .await
                    .expect(GET_MASTERIES_ERR)
                    .iter()
                    .map(|f| ChampionMasteryDisplay::with(f.clone()))
                    .collect();
                for m in masteries {
                    println!("{}", m);
                }
    }
}
async fn print_game(api_key: String, route: PlatformRoute, names: Vec<String>, game: usize) {
    for name in names {
    let id = get_summoner(&api_key, route, &name)
                    .await
                    .expect(&GET_SUMMONER_ERR)
                    .expect(&SUMMONER_IS_NONE)
                    .puuid;
                let matches = get_games(&api_key, route, &id).await.expect(&GET_GAMES_ERR);
                let m = MatchDisplay::with(
                    matches
                        .get(game)
                        .unwrap()
                        .clone(),
                );
                println!("{}", m);
    }

}
