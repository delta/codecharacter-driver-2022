use serde::de;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Attacker {
    pub id: u32,
    pub hp: u32,
    pub range: u32,
    pub attack_power: u32,
    pub speed: u32,
    pub price: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Defender {
    pub id: u32,
    pub hp: u32,
    pub range: u32,
    pub attack_power: u32,
    pub price: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GameParameters {
    pub attackers: Vec<Attacker>,
    pub defenders: Vec<Defender>,
    pub no_of_turns: u32,
    pub no_of_coins: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Language {
    C,
    CPP,
    JAVA,
    PYTHON,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GameRequest {
    pub game_id: String,
    pub parameters: GameParameters,
    pub source_code: String,
    pub language: Language,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub map: Vec<Vec<u8>>,
}

// Reference: https://serde.rs/attr-bound.html
fn deserialize_from_str<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {

    use super::{Attacker, Defender, GameParameters, GameRequest};
    #[test]
    pub fn deserealization_test() {
        // An example request that we might get from backend
        let example_request = r#"{"game_id":"0fa0f12d-d472-42d5-94b4-011e0c916023","parameters":{"attackers":[{"id":1,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1},{"id":2,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1}],"defenders":[{"id":1,"hp":10,"range":4,"attack_power":5,"price":1},{"id":2,"hp":10,"range":6,"attack_power":5,"price":1}],"no_of_turns":500,"no_of_coins":1000},"source_code":"print(x)","language":"PYTHON","map":"[[1,0],[0,2]]"}"#;

        let expected_deserealized_struct = GameRequest {
            game_id: "0fa0f12d-d472-42d5-94b4-011e0c916023".to_owned(),
            parameters: GameParameters {
                attackers: vec![
                    Attacker {
                        id: 1,
                        hp: 10,
                        range: 3,
                        attack_power: 3,
                        speed: 3,
                        price: 1,
                    },
                    Attacker {
                        id: 2,
                        hp: 10,
                        range: 3,
                        attack_power: 3,
                        speed: 3,
                        price: 1,
                    },
                ],
                defenders: vec![
                    Defender {
                        id: 1,
                        hp: 10,
                        range: 4,
                        attack_power: 5,
                        price: 1,
                    },
                    Defender {
                        id: 2,
                        hp: 10,
                        range: 6,
                        attack_power: 5,
                        price: 1,
                    },
                ],
                no_of_turns: 500,
                no_of_coins: 1000,
            },
            language: super::Language::PYTHON,
            source_code: r#"print(x)"#.to_owned(),
            map: vec![vec![1, 0], vec![0, 2]],
        };
        let deserealized_example_request: GameRequest =
            serde_json::from_str(example_request).unwrap();
        assert_eq!(deserealized_example_request, expected_deserealized_struct);
    }
}
