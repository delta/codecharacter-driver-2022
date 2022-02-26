#include "player_code.h"

#include <algorithm>
#include <cmath>
#include <compare>
#include <iostream>
#include <set>
#include <sstream>
#include <unordered_map>
#include <vector>

Attributes::Attributes(unsigned hp, unsigned range, unsigned attack_power,
                       unsigned speed, unsigned price)
    : hp(hp), range(range), attack_power(attack_power), speed(speed),
      price(price) {}

Position::Position(int x, int y) : _x(x), _y(y) {}

int Position::get_x() const { return _x; }
int Position::get_y() const { return _y; }

double Position::distance_to(Position other) const {
  auto delta_x = other.get_x() - this->get_x();
  auto delta_y = other.get_y() - this->get_y();
  return sqrt((double)(delta_x * delta_x + delta_y * delta_y));
}

bool is_valid_spawn_position(int x, int y) {
  if (x < 0 || y < 0 || x >= static_cast<int>(Constants::MAP_NO_OF_COLS) ||
      y >= static_cast<int>(Constants::MAP_NO_OF_ROWS)) {
    return false;
  }
  return x == 0 || y == 0 ||
         (x == (static_cast<int>(Constants::MAP_NO_OF_COLS) - 1)) ||
         (y == (static_cast<int>(Constants::MAP_NO_OF_ROWS) - 1));
}

bool is_valid_spawn_position(Position pos) {
  return is_valid_spawn_position(pos.get_x(), pos.get_y());
}

std::vector<Position> get_all_valid_spawn_positions() {
  std::vector<Position> all_valid_positions;
  // x is 0
  for (int j = 0; j < static_cast<int>(Constants::MAP_NO_OF_ROWS); j++) {
    all_valid_positions.push_back({0, j});
  }

  // y is 0
  for (int i = 1; i < static_cast<int>(Constants::MAP_NO_OF_COLS); i++) {
    all_valid_positions.push_back({i, 0});
  }

  // x is MAP_NO_OF_ROWS-1
  for (int j = 1; j < static_cast<int>(Constants::MAP_NO_OF_ROWS); j++) {
    all_valid_positions.push_back(
        {static_cast<int>(Constants::MAP_NO_OF_COLS) - 1, j});
  }

  // y is MAP_NO_OF_COLS - 1
  for (int i = 1; (i + 1) < static_cast<int>(Constants::MAP_NO_OF_COLS); i++) {
    all_valid_positions.push_back(
        {i, static_cast<int>(Constants::MAP_NO_OF_ROWS) - 1});
  }
  return all_valid_positions;
}

Actor::Actor(size_t id, size_t hp, size_t type, Position pos)
    : _id(id), _hp(hp), _type(type), _position(pos) {}
size_t Actor::get_id() const { return _id; }
size_t Actor::get_hp() const { return _hp; }
size_t Actor::get_type() const { return _type; }
Position Actor::get_position() const { return _position; }

Attacker::Attacker(size_t id, size_t hp, size_t type, Position pos)
    : Actor(id, hp, type, pos) {}

Defender::Defender(size_t id, size_t hp, size_t type, Position pos)
    : Actor(id, hp, type, pos) {}

State::State(std::vector<Attacker> attackers, std::vector<Defender> defenders,
             size_t no_of_coins_left, size_t turn_no)
    : _turn_no(turn_no), _no_of_coins_left(no_of_coins_left),
      _attackers(std::move(attackers)), _defenders(std::move(defenders)) {}

const std::vector<Attacker> &State::get_attackers() const {
  return this->_attackers;
}
const std::vector<Defender> &State::get_defenders() const {
  return this->_defenders;
}
size_t State::get_turn_no() const { return this->_turn_no; }
size_t State::get_coins_left() const { return this->_no_of_coins_left; }

Game::Game() {}

void Game::spawn_attacker(size_t id, Position pos) {
  this->_spawn_postions.push_back({id, pos});
  this->_already_spawned_positions.insert(pos);
}
bool Game::already_spawned_at_position(Position pos) {
  return this->_already_spawned_positions.contains(pos);
}
void Game::set_target(size_t attacker_id, size_t defender_id) {
  this->_player_set_targets.insert({attacker_id, defender_id});
}
void Game::set_target(const Attacker &attacker, const Defender &defender) {
  this->_player_set_targets.insert({attacker.get_id(), defender.get_id()});
}

std::ostringstream &Game::logr() { return this->_logr; }

const std::unordered_map<size_t, size_t> &Game::get_player_set_targets() const {
  return this->_player_set_targets;
}
const std::vector<std::pair<size_t, Position>> &
Game::get_spawn_positions() const {
  return this->_spawn_postions;
}
const std::set<Position> &Game::get_already_spawned_positions() const {
  return this->_already_spawned_positions;
}

Map::Map(std::vector<std::vector<int>> map_as_grid)
    : _grid(move(map_as_grid)) {}

Map Map::get(std::istream &stream) {
  static int no_of_times_called = 0;
  no_of_times_called++;
  if (no_of_times_called > 1) {
    throw std::runtime_error(
        "Player tried to call an internal function. Not allowed");
  }

  size_t m = 0;
  size_t n = 0;
  stream >> m;
  stream >> n;

  Map::no_of_rows = m;
  Map::no_of_cols = n;

  std::vector<std::vector<int>> grid(m, std::vector<int>(n, 0));
  for (auto &row : grid) {
    for (auto &cell : row) {
      stream >> cell;
    }
  }
  return {move(grid)};
}

std::vector<Defender> Map::spawn_defenders() const {
  static int no_of_times_called = 0;
  no_of_times_called++;
  if (no_of_times_called > 1) {
    throw std::runtime_error(
        "Player tried to call an internal function. Not allowed");
  }

  std::vector<Defender> defenders;
  size_t id = 0;
  for (size_t x = 0; x < this->no_of_cols; x++) {
    for (size_t y = 0; y < this->no_of_rows; y++) {
      if (this->_grid[y][x] > 0) {
        auto &attributes_for_defender =
            Constants::DEFENDER_TYPE_ATTRIBUTES.at(this->_grid[y][x]);
        defenders.emplace_back(id++, attributes_for_defender.hp,
                               this->_grid[y][x], Position(x, y));
      }
    }
  }
  return defenders;
}
