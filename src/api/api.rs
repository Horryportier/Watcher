use riven::consts::PlatformRoute;
use riven::models::league_v4::LeagueEntry;
use riven::models::summoner_v4::Summoner;
use riven::{RiotApi, RiotApiError};

const API_KEY: &'static str = std::env!("RGAPI_KEY");

pub async fn get_rank(route: PlatformRoute, id: &str) -> Result<Vec<LeagueEntry>, RiotApiError> {
    RiotApi::new(API_KEY)
        .league_v4()
        .get_league_entries_for_summoner(route, id)
        .await
}

pub async fn get_summoner(
    route: PlatformRoute,
    name: &str,
) -> Result<Option<Summoner>, RiotApiError> {
    let res = RiotApi::new(API_KEY)
        .summoner_v4()
        .get_by_summoner_name(route, name)
        .await;
    match res {
        Ok(op) => match op {
            None => return Ok(None),
            Some(s) => return Ok(Some(s)),
        },
        Err(e) => return Err(e),
    }
}
