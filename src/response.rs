use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum GameStatusEnum {
    IDLE,
    EXECUTING,
    EXECUTED,
    EXECUTE_ERROR,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GameResult {
    pub destruction_percentage: f64,
    pub coins_used: u64,
    pub has_errors: bool,
    pub log: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct GameStatus {
    pub game_id: String,
    pub game_status: GameStatusEnum,
    pub game_result: Option<GameResult>,
}

#[cfg(test)]
mod tests {

    use super::{GameStatus, GameStatusEnum};
    #[test]
    pub fn serialization_test() {
        // An example respone
        let expected_response = r#"{"game_id":"030af985-f4b5-4914-94d8-e559576449e3","game_status":"EXECUTING","game_result":null}"#;

        let game_status = GameStatus {
            game_id: "030af985-f4b5-4914-94d8-e559576449e3".to_string(),
            game_status: GameStatusEnum::EXECUTING,
            game_result: None,
        };

        let serialized_game_status = serde_json::to_string(&game_status).unwrap();

        assert_eq!(serialized_game_status, expected_response);
    }
}
