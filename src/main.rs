use std::env::args;

use api::api::{get_rank, get_summoner};
use crossterm::style::{style, Color, Colors, Print, PrintStyledContent, StyledContent, Stylize};
use riven::{
    consts::{PlatformRoute, Tier},
    RiotApiError, models::league_v4::LeagueEntry,
};
use ui::ui::ui;
use utils::LeagueEntryDisplay;

mod api;
mod ui;
mod utils;

const ROUTE: PlatformRoute = PlatformRoute::EUN1;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        ui();
    }

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "rank" => {
                let id = get_summoner(ROUTE, &args[i + 1])
                    .await
                    .expect("oeu")
                    .expect("")
                    .id;
                let res = get_rank(ROUTE, id.as_str()).await.expect("oeau");
                let ranks: Vec<LeagueEntryDisplay> =  res.iter().map(|f| LeagueEntryDisplay{ league_entry: f.clone()}).collect();
                for r in ranks {
                    print!("{}", r)
                }
            }
            _ => (),
        }
    }
    Ok(())
}
