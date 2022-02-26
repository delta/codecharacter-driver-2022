import sys
from run import Position, Attacker, Defender, Constants, Map, State, Game, run


def output(state: State, game: Game):
    log_line = game.get_log()
    if log_line:
        sys.stderr.write(f"TURN {state.turn_no}\n")
        sys.stderr.write(log_line)
        sys.stderr.write(f"ENDLOG\n")

    sys.stdout.write(f"{len(game.spawn_positions)}\n")
    for id, position in game.spawn_positions:
        sys.stdout.write(f"{id} {position.x} {position.y}\n")

    sys.stdout.write(f"{len(game.player_set_targets)}\n")
    for attacker_id, defender_id in game.player_set_targets:
        sys.stdout.write(f"{attacker_id} {defender_id}\n")


def next_state(cur_turn_no: int) -> State:
    no_of_active_attackers = int(sys.stdin.readline())
    attackers = []

    for _ in range(no_of_active_attackers):
        id, x, y, a_type, hp = map(int, sys.stdin.readline().split())
        attackers.append(
            Attacker(id, hp, Constants.ATTACKER_TYPE_ATTRIBUTES[a_type], Position(x, y))
        )

    no_of_active_defenders = int(sys.stdin.readline())
    defenders = []

    for _ in range(no_of_active_defenders):
        id, x, y, d_type, hp = map(int, sys.stdin.readline().split())
        defenders.append(
            Defender(id, hp, Constants.DEFENDER_TYPE_ATTRIBUTES[d_type], Position(x, y))
        )

    no_of_coins_left = int(sys.stdin.readline())

    return State(attackers, defenders, no_of_coins_left, cur_turn_no + 1)


Constants.initialize()
Map.initialize()

state = State([], Map.spawn_defenders(), Constants.MAX_NO_OF_COINS, 0)

game = run(state)

output(state, game)

for i in range(Constants.NO_OF_TURNS):
    state = next_state(state.turn_no)
    game = run(state)
    output(state, game)
