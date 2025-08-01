use serde::Deserialize;
use std::error::Error;
use solarized::{
    print_fancy, PrintMode::NewLine,
    VIOLET, BLUE, CYAN, GREEN,
    //YELLOW, ORANGE, RED, MAGENTA, GREY
};

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
    print_fancy(&[
        ("Fetching Warframe open world cycle status...", CYAN, vec![]),
    ], NewLine);
    let response = reqwest::get(API_URL).await?;
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }
    let body = response.text().await?;
    let world_state: WorldState = serde_json::from_str(&body)?;
    let cetus_state = if world_state.cetus_cycle.is_day { "Day" } else { "Night" };
    let vallis_state = if world_state.vallis_cycle.is_warm { "Warm" } else { "Cold" };
    print_fancy(&[
        ("\n-----\n", CYAN, vec![]),
        ("Plains of Eidolon (Cetus)\n", VIOLET, vec![]),
        ("  State: ", CYAN, vec![]),
        (&format!("{}\n", cetus_state), GREEN, vec![]),
        ("  Time until change: ", CYAN, vec![]),
        (&world_state.cetus_cycle.time_left, BLUE, vec![]),
        ("\n-----\n", CYAN, vec![]),
        ("Orb Vallis\n", VIOLET, vec![]),
        ("  State: ", CYAN, vec![]),
        (&format!("{}\n", vallis_state), GREEN, vec![]),
        ("  Time until change: ", CYAN, vec![]),
        (&world_state.vallis_cycle.time_left, BLUE, vec![]),
        ("\n-----\n", CYAN, vec![]),
        ("Cambion Drift (Deimos)\n", VIOLET, vec![]),
        ("  State: ", CYAN, vec![]),
        (&format!("{}\n", world_state.cambion_cycle.active.to_uppercase()), GREEN, vec![]),
        ("  Time until change: ", CYAN, vec![]),
        (&world_state.cambion_cycle.time_left, BLUE, vec![]),
        ("\n-----", CYAN, vec![]),
    ], NewLine);
    /*
    println!("\n----------------------------------------");
    println!(" Plains of Eidolon (Cetus)");
    println!("   State: {}", cetus_state);
    println!("   Time until change: {}", world_state.cetus_cycle.time_left);
    println!("\n----------------------------------------");
    println!(" Orb Vallis");
    println!("   State: {}", vallis_state);
    println!("   Time until change: {}", world_state.vallis_cycle.time_left);
    println!("\n----------------------------------------");
    println!(" Cambion Drift (Deimos)");
    println!("   State: {}", world_state.cambion_cycle.active.to_uppercase());
    println!("   Time until change: {}", world_state.cambion_cycle.time_left);
    println!("\n----------------------------------------");
*/
    Ok(())
}
