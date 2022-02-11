use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize,Deserialize)]
struct TroopType{
    ttid: u32,
    hp: u32,
    speed: u32,
    atk_power: u32,
    range: u32,
    price: u32
}

#[derive(Serialize,Deserialize)]
struct DefenseType{
    dtid: u32,
    hp: u32,
    atk_power: u32,
    range: u32
}
#[derive(Serialize,Deserialize)]
pub struct MatchRequest{
    player_code_language : String,
    player_code : String,
    map_dimensions : String,
    pub map : String,
    no_of_turns: u32,
    no_of_coins: u32,
    troop_types: [TroopType; 3],
    defense_types: [DefenseType; 3],
}