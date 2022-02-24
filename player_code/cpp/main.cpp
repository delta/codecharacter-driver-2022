#include "player_code.h"

void init_constants() {

  std::cin >> Constants::NO_OF_TURNS >> Constants::MAX_NO_OF_COINS;

  // All the attacker types
  std::cin >> Constants::NO_OF_ATTACKER_TYPES;

  std::unordered_map<size_t, Attributes> attacker_type_to_attributes;
  for (size_t i = 1; i <= Constants::NO_OF_ATTACKER_TYPES; i++) {
    unsigned hp, range, attack_power, speed, price;
    std::cin >> hp >> range >> attack_power >> speed >> price;
    attacker_type_to_attributes.insert(
        std::make_pair(i, Attributes(hp, range, attack_power, speed, price)));
  }
  Constants::ATTACKER_TYPE_ATTRIBUTES = attacker_type_to_attributes;

  // All the defender types
  std::cin >> Constants::NO_OF_DEFENDER_TYPES;

  std::unordered_map<size_t, Attributes> defender_type_to_attributes;
  for (size_t i = 1; i <= Constants::NO_OF_DEFENDER_TYPES; i++) {
    unsigned hp, range, attack_power, speed, price;
    std::cin >> hp >> range >> attack_power >> speed >> price;
    defender_type_to_attributes.insert(
        std::make_pair(i, Attributes(hp, range, attack_power, speed, price)));
  }
  Constants::DEFENDER_TYPE_ATTRIBUTES = defender_type_to_attributes;
}

Map get_initial_map() {
  auto map = Map::get(std::cin);
  Constants::MAP_NO_OF_ROWS = map.no_of_rows;
  Constants::MAP_NO_OF_COLS = map.no_of_rows;
  return map;
}

void output(State &state, Game &game) {

  // Player logs are logged to cerr, so that driver will collect it
  game.logr().flush();
  std::cerr << "TURN " << state.get_turn_no() << '\n';
  std::cerr << game.logr().view() << '\n';
  std::cerr << "ENDLOG" << std::endl;

  // Game details logged
  const auto &spawn_positions = game.get_spawn_positions();
  const auto &player_set_targets = game.get_player_set_targets();

  std::cout << spawn_positions.size() << std::endl;
  for (const auto &entry : spawn_positions) {
    std::cout << entry.first << " " << entry.second.get_x() << " "
              << entry.second.get_y() << std::endl;
  }

  std::cout << player_set_targets.size() << std::endl;
  for (const auto &entry : player_set_targets) {
    std::cout << entry.first << " " << entry.second << std::endl;
  }
}

void dump_logs(State &state, Game &game) {}

State next_state(size_t cur_turn_no) {
  size_t no_of_active_defenders;
  size_t no_of_active_attackers;

  std::cin >> no_of_active_attackers;
  std::vector<Attacker> attackers;
  for (size_t i = 0; i < no_of_active_attackers; i++) {
    size_t id, hp, x, y, type;
    std::cin >> id >> x >> y >> type >> hp;
    attackers.push_back(Attacker(id, hp, type, Position(x, y)));
  }

  std::cin >> no_of_active_defenders;
  std::vector<Defender> defenders;
  for (size_t i = 0; i < no_of_active_defenders; i++) {
    size_t id, hp, x, y, type;
    std::cin >> id >> x >> y >> type >> hp;
    defenders.push_back(Defender(id, hp, type, Position(x, y)));
  }

  size_t coins_left;
  std::cin >> coins_left;

  return {move(attackers), move(defenders), coins_left, cur_turn_no + 1};
}

int main() {
  init_constants();
  Map initial_map = get_initial_map();

  State state({}, initial_map.spawn_defenders(), Constants::MAX_NO_OF_COINS, 0);

  auto game = run(state);

  output(state, game);

  for (size_t i = 0; i < Constants::NO_OF_TURNS; i++) {
    state = next_state(state.get_turn_no());
    game = run(state);
    output(state, game);
  }
}
