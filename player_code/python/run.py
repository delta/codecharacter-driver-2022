from player_code import (
    Position,
    Attacker,
    Defender,
    AttackerType,
    DefenderType,
    Constants,
    Map,
    State,
    Game,
    is_valid_spawn_position,
    get_all_valid_spawn_positions,
)


def run(state: State) -> Game:
    game = Game()
    for valid_spawn_position in get_all_valid_spawn_positions():
        game.spawn_attacker(1, valid_spawn_position)
    return game
