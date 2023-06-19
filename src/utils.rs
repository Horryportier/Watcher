use std::fmt::Display;
use chrono;

use crossterm::style::Stylize;
use riven::consts::PlatformRoute;

pub const ROUTE_NAMES: [&str; 15]= ["kr", "ru", "br", "jp", "la1", "la2", "na", "oce", "ph", "sg", "th", "tr", "tw", "eune", "euw"];

#[macro_export]
macro_rules! err_print  {
    ($($token:tt)*) => {
        format!("{}",  $(stringify!($token).with(crossterm::style::Color::Red),),* )
    };
}


pub fn  print_help() {
    const TEXT: &str = r###"
            Watcher 
______________________________

Usage: Watcher [name] [region] [flags]


# Regions:
["kr", "ru", "br", "jp", "la1", "la2", "na", "oce", "ph", "sg", "th", "tr", "tw", "eune", "euw"]
Flags:
------------------------------
NONE     lunches Tui 
-h | --help     prints help
-s | --summoner searches for summoner
-r | --rank     get's summoner rank 
-m | --mastery  get's first 10 highest champions mastery's
-g | --game     -g 0..20 get's game from 20 games
        "###;

    println!("{}", TEXT);
}


pub fn routes() -> Vec<(String, PlatformRoute)> {
    let mut map: Vec<(String, PlatformRoute)> = vec![];
        map.push(("kr".to_string(), PlatformRoute::KR));
        map.push(("ru".to_string(), PlatformRoute::RU));
        map.push(("br".to_string(), PlatformRoute::BR1));
        map.push(("jp".to_string(), PlatformRoute::JP1));
        map.push(("la1".to_string(), PlatformRoute::LA1));
        map.push(("la2".to_string(), PlatformRoute::LA2));
        map.push(("na".to_string(), PlatformRoute::NA1));
        map.push(("oce".to_string(), PlatformRoute::OC1));
        map.push(("ph".to_string(), PlatformRoute::PH2));
        map.push(("sg".to_string(), PlatformRoute::SG2));
        map.push(("th".to_string(), PlatformRoute::TH2));
        map.push(("tr".to_string(), PlatformRoute::TR1));
        map.push(("tw".to_string(), PlatformRoute::TW2));
        map.push(("eune".to_string(), PlatformRoute::EUN1));
        map.push(("euw".to_string(), PlatformRoute::EUW1));
        map
}

pub  fn parse_route(key: String) -> PlatformRoute {
    let routes = routes();
                routes
                .iter()
                .filter(|f| key == f.0)
                .map(|f| f.1)
                .collect::<Vec<PlatformRoute>>()
                .pop()
                .unwrap_or(PlatformRoute::KR)

}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum  LogKind {
    Info,
    Warning,
    Error,
}


#[derive(Debug, Clone)]
pub struct Log {
pub kind: LogKind,
pub time_stamp: String,
pub msg: String
}

impl  Log {
    pub fn new(kind: LogKind, msg: String) -> Log {                                                                                                                                             
        let now = chrono::offset::Local::now(); 
        let time_stamp = now.date_naive().to_string();
        Log { kind, time_stamp, msg}
    }
}

impl  Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log = self.clone();
        let msg = match log.kind {
                    LogKind::Info => log.msg.with(crossterm::style::Color::Green),
                    LogKind::Warning => log.msg.with(crossterm::style::Color::Yellow),
                    LogKind::Error =>  log.msg.with(crossterm::style::Color::Red)
                };
        let text = format!("{:?}=> {}:{}",log.kind, 
                log.time_stamp.with(crossterm::style::Color::Magenta), msg);

        write!(f, "{}", text)
    }
}

pub fn is_numeric(input: &str) -> bool {
    match input.parse::<usize>(){
        Ok(_) => true,
        Err(..) => false,
    }
}
