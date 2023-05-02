//use riven::consts::{PlatformRoute, RegionalRoute, RegionalRouteIter};
//use riven::models::match_v5::Match;
//use riven::RiotApi;

use std::env::args;

use ui::ui::ui;

mod ui;

//#[tokio::main]
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        ui();
    }

    for (i ,arg) in args.iter().enumerate() {
        match arg.as_str() {
           _  => {}
        }
    }
    
    // Enter tokio async runtime.
//    let rt = tokio::runtime::Runtime::new().unwrap();
//    rt.block_on(async {
//        // Create RiotApi instance from key string.
//        let api_key = std::env!("RGAPI_KEY"); // "RGAPI-01234567-89ab-cdef-0123-456789abcdef";
//        let riot_api = RiotApi::new(api_key);
//
//        // Get summoner data.
//        let summoner = riot_api
//            .summoner_v4()
//            .get_by_summoner_name(PlatformRoute::EUN1, "HorryPortier6")
//            .await
//            .expect("Get summoner failed.")
//            .expect("There is no summoner with that name.");
//
//        // Print summoner name.
//        println!("{:?} Champion Masteries:", summoner);
//
//        // Get champion mastery data.
//        let _masteries = riot_api
//            .champion_mastery_v4()
//            .get_all_champion_masteries(PlatformRoute::NA1, &summoner.id)
//            .await
//            .expect("Get champion masteries failed.");
//
//        // Print champion masteries.
//       // for (i, mastery) in masteries.iter().enumerate() {
//       //     println!(
//       //         "{: >2}) {: <9}    {: >7} ({})",
//       //         i + 1,
//       //         mastery.champion_id.name().unwrap_or("UNKNOWN"),
//       //         mastery.champion_points,
//       //         mastery.champion_level
//       //     );
//       // }
//
//        let ids = riot_api
//            .match_v5()
//            .get_match_ids_by_puuid(
//                RegionalRoute::EUROPE,
//                &summoner.puuid,
//                None,
//                None,
//                Some(riven::consts::Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO),
//                None,
//                None,
//                None,
//            )
//            .await
//            .expect("failed to get match");
//
//        //println!("{:?}", ids);
//
//        let mut games: Vec<Option<Match>> = vec![];
//        for id in ids {
//            games.push(
//                riot_api
//                    .match_v5()
//                    .get_match(RegionalRoute::EUROPE, &id)
//                    .await
//                    .expect("couldn't get game data"),
//            );
//        }
//
//        println!("GAMES: {:?}", games[0])
//    });
}
