use std::env::args;

use api::api::{get_rank, get_summoner};
use riven::consts::PlatformRoute;
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
        let _ = ui().await;
    }

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "rank" => {
                let id = get_summoner(ROUTE, &args[i + 1])
                    .await
                    .expect("couldn't get_summoner")
                    .expect("get_summoner is none")
                    .id;
                let res = get_rank(ROUTE, id.as_str()).await.expect("couldn't get_rank");
                let ranks: Vec<LeagueEntryDisplay> =
                    res.iter().map(|f| LeagueEntryDisplay(f.clone())).collect();
                for r in ranks {
                    print!("{}", r)
                }
            }
            _ => (),
        }
    }
    Ok(())
}
