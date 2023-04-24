use riven::consts::PlatformRoute;
use riven::RiotApi;

fn main() {
    // Enter tokio async runtime.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Create RiotApi instance from key string.
        let api_key = std::env!("RGAPI_KEY"); // "RGAPI-01234567-89ab-cdef-0123-456789abcdef";
        let riot_api = RiotApi::new(api_key);

        // Get summoner data.
        let summoner = riot_api
            .summoner_v4()
            .get_by_summoner_name(PlatformRoute::NA1, "잘 못")
            .await
            .expect("Get summoner failed.")
            .expect("There is no summoner with that name.");

        // Print summoner name.
        println!("{} Champion Masteries:", summoner.name);

        // Get champion mastery data.
        let masteries = riot_api
            .champion_mastery_v4()
            .get_all_champion_masteries(PlatformRoute::NA1, &summoner.id)
            .await
            .expect("Get champion masteries failed.");

        // Print champion masteries.
        for (i, mastery) in masteries.iter().take(10).enumerate() {
            println!(
                "{: >2}) {: <9}    {: >7} ({})",
                i + 1,
                mastery.champion_id.name().unwrap_or("UNKNOWN"),
                mastery.champion_points,
                mastery.champion_level
            );
        }
    });
}
