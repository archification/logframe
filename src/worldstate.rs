use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WorldState {
    cetus_cycle: CetusCycle,
    vallis_cycle: VallisCycle,
    cambion_cycle: CambionCycle,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CetusCycle {
    is_day: bool,
    time_left: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VallisCycle {
    is_warm: bool,
    time_left: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CambionCycle {
    active: String,
    time_left: String,
}

pub async fn worldstate() -> Result<(), Box<dyn Error>> {
    const API_URL: &str = "https://api.warframestat.us/pc/";
    println!("Fetching Warframe open world cycle status...");
    let response = reqwest::get(API_URL).await?;
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }
    let body = response.text().await?;
    let world_state: WorldState = serde_json::from_str(&body)?;
    println!("\n----------------------------------------");
    let cetus_state = if world_state.cetus_cycle.is_day { "Day" } else { "Night" };
    println!(" Plains of Eidolon (Cetus)");
    println!("   State: {}", cetus_state);
    println!("   Time until change: {}", world_state.cetus_cycle.time_left);
    println!("\n----------------------------------------");
    let vallis_state = if world_state.vallis_cycle.is_warm { "Warm" } else { "Cold" };
    println!(" Orb Vallis");
    println!("   State: {}", vallis_state);
    println!("   Time until change: {}", world_state.vallis_cycle.time_left);
    println!("\n----------------------------------------");
    println!(" Cambion Drift (Deimos)");
    println!("   State: {}", world_state.cambion_cycle.active.to_uppercase());
    println!("   Time until change: {}", world_state.cambion_cycle.time_left);
    println!("\n----------------------------------------");
    Ok(())
}
