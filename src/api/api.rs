use std::collections::HashMap;

use riven::consts::{PlatformRoute, RegionalRoute};
use riven::models::champion_mastery_v4::ChampionMastery;
use riven::models::league_v4::LeagueEntry;
use riven::models::match_v5::Match;
use riven::models::summoner_v4::Summoner;
use riven::{RiotApi, RiotApiError};

pub async fn get_rank(
    api_key: &str,
    route: PlatformRoute,
    id: &str,
) -> Result<Vec<LeagueEntry>, RiotApiError> {
    RiotApi::new(api_key)
        .league_v4()
        .get_league_entries_for_summoner(route, id)
        .await
}

pub async fn get_summoner(
    api_key: &str,
    route: PlatformRoute,
    name: &str,
) -> Result<Option<Summoner>, RiotApiError> {
    let res = RiotApi::new(api_key)
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

pub async fn get_masteries(
    api_key: &str,
    route: PlatformRoute,
    id: &str,
    top: usize,
) -> Result<Vec<ChampionMastery>, RiotApiError> {
    let res = RiotApi::new(api_key)
        .champion_mastery_v4()
        .get_top_champion_masteries(route, id, Some(top as i32))
        .await;
    match res {
        Ok(i) => {
            return Ok(i);
        }
        Err(e) => return Err(e),
    }
}

pub async fn get_games(
    api_key: &str,
    route: PlatformRoute,
    puuid: &str,
) -> Result<Vec<Match>, RiotApiError> {
    let mut remap = HashMap::<Vec<PlatformRoute>, RegionalRoute>::new();
    remap.insert(
        [PlatformRoute::NA1, PlatformRoute::BR1].to_vec(),
        RegionalRoute::AMERICAS,
    );
    remap.insert(
        [PlatformRoute::EUN1, PlatformRoute::EUW1].to_vec(),
        RegionalRoute::EUROPE,
    );
    remap.insert(
        [PlatformRoute::KR, PlatformRoute::JP1, PlatformRoute::TW2].to_vec(),
        RegionalRoute::ASIA,
    );
    let riot = RiotApi::new(api_key);

    let res = riot
        .match_v5()
        .get_match_ids_by_puuid(
            *remap.get(&vec![route]).unwrap_or(&RegionalRoute::EUROPE),
            puuid,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    let ids = match res {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    let mut matches: Vec<Match> = vec![];
    for id in ids.iter() {
        let res = riot
            .match_v5()
            .get_match(
                *remap.get(&vec![route]).unwrap_or(&RegionalRoute::EUROPE),
                &id,
            )
            .await;
        match res {
            Ok(i) => match i {
                Some(m) => matches.push(m),
                None => {}
            },
            Err(e) => return Err(e),
        }
    }
    Ok(matches)
}
