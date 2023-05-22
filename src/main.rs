use std::env::args;

use api::api::{get_games, get_masteries, get_rank, get_summoner};
use crossterm::style::Stylize;
use display::{ChampionMasteryDisplay, LeagueEntryDisplay, MatchDisplay, SummonerDisplay, With};
use riven::consts::PlatformRoute;
use ui::ui::ui;
use utils::print_help;

mod api;
mod display;
mod ui;
mod utils;

const ROUTE: PlatformRoute = PlatformRoute::EUN1;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args: Vec<String> = args().collect();

    if args.len() == 1 {
        let _ = ui().await;
    };

    let get_summoner_err: String = err_print!("couldn't get_summoner");
    let summoner_is_none: String = err_print!("summoner is none");
    let get_rank_err: String = err_print!("couldn't get rank");
    let get_masteries_err: String = err_print!("couldn't get masteries");
    let get_games_err: String = err_print!("couldn't get games");

    let name: &str = &args[1].clone();
    args.remove(1);
    args.remove(0);
    //let route: &str = &args[2];

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            "--sum" | "-s" => {
                let sum = get_summoner(ROUTE, name)
                    .await
                    .expect(&get_summoner_err)
                    .expect(&summoner_is_none);
                println!("{}", SummonerDisplay::with(sum));
            }
            "--rank" | "-r" => {
                let id = get_summoner(ROUTE, name)
                    .await
                    .expect(&get_summoner_err)
                    .expect(&summoner_is_none)
                    .id;
                let res = get_rank(ROUTE, id.as_str())
                    .await
                    .expect(get_rank_err.as_str());
                let ranks: Vec<LeagueEntryDisplay> = res
                    .iter()
                    .map(|f| LeagueEntryDisplay::with(f.clone()))
                    .collect();
                for r in ranks {
                    print!("{}", r);
                }
            }
            "--mastery" | "-m" => {
                let id = get_summoner(ROUTE, name)
                    .await
                    .expect(&get_summoner_err)
                    .expect(&summoner_is_none)
                    .id;
                let masteries: Vec<ChampionMasteryDisplay> = get_masteries(ROUTE, &id, 10)
                    .await
                    .expect(get_masteries_err.as_str())
                    .iter()
                    .map(|f| ChampionMasteryDisplay::with(f.clone()))
                    .collect();
                for m in masteries {
                    println!("{}", m);
                }
            }
            "--game" | "-g" => {
                let id = get_summoner(ROUTE, name)
                    .await
                    .expect(&get_summoner_err)
                    .expect(&summoner_is_none)
                    .puuid;
                let matches = get_games(ROUTE, &id).await.expect(&get_games_err);
                let m = MatchDisplay::with(
                    matches
                        .get(args[i + 1].parse().unwrap_or(0))
                        .unwrap()
                        .clone(),
                );
                println!("{}", m);
            }
            _ => {}
        }
    }
    Ok(())
}
