use std::env::args;

use api::api::{get_games, get_masteries, get_rank, get_summoner};
use riven::consts::PlatformRoute;
use ui::ui::ui;
use utils::{ChampionMasteryDisplay, LeagueEntryDisplay, SummonerDisplay, MatchDisplay};

mod api;
mod ui;
mod utils;

const ROUTE: PlatformRoute = PlatformRoute::EUN1;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        let _ = ui().await;
    }

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "sum" => {
                let sum = get_summoner(ROUTE, &args[i + 1])
                    .await
                    .expect("couldn't get_summoner")
                    .expect("get_summoner is none");
                println!("{}", SummonerDisplay::with(sum));
            }
            "rank" => {
                let id = get_summoner(ROUTE, &args[i + 1])
                    .await
                    .expect("couldn't get_summoner")
                    .expect("get_summoner is none")
                    .id;
                let res = get_rank(ROUTE, id.as_str())
                    .await
                    .expect("couldn't get_rank");
                let ranks: Vec<LeagueEntryDisplay> = res
                    .iter()
                    .map(|f| LeagueEntryDisplay::with(f.clone()))
                    .collect();
                for r in ranks {
                    print!("{}", r)
                }
            }
            "mastery" => {
                let id = get_summoner(ROUTE, &args[i + 1])
                    .await
                    .expect("couldn't get_summoner")
                    .expect("get_summoner is none")
                    .id;
                let masteries: Vec<ChampionMasteryDisplay> = get_masteries(ROUTE, &id, 10)
                    .await
                    .expect("couldn't get masteries")
                    .iter()
                    .map(|f| ChampionMasteryDisplay::with(f.clone()))
                    .collect();
                for m in masteries {
                    println!("{}", m)
                }
            }
            "game" => {
                let id = get_summoner(ROUTE, &args[i + 1])
                    .await
                    .expect("couldn't get_summoner")
                    .expect("get_summoner is none")
                    .puuid;
                let matches = get_games(ROUTE, &id).await.expect("couldn't get_games");
                let m = MatchDisplay::with(matches.get(args[i + 2].parse().unwrap_or(0)).unwrap().clone());
                println!("{}", m)
            }
            _ => (),
        }
    }
    Ok(())
}
