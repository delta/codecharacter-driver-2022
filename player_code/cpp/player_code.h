#pragma once

#include <compare>
#include <iostream>
#include <set>
#include <sstream>
#include <unordered_map>
#include <vector>

struct Attributes {
  const unsigned hp;
  const unsigned range;
  const unsigned attack_power;
  const unsigned speed;
  const unsigned price;
  Attributes(unsigned hp, unsigned range, unsigned attack_power, unsigned speed,
             unsigned price);
};

struct Constants {
static inline size_t MAP_NO_OF_ROWS;
static inline size_t MAP_NO_OF_COLS;
static inline size_t NO_OF_DEFENDER_TYPES;
static inline size_t NO_OF_ATTACKER_TYPES;
static inline size_t NO_OF_TURNS;
static inline size_t MAX_NO_OF_COINS;

static inline std::unordered_map<size_t, Attributes> ATTACKER_TYPE_ATTRIBUTES;
static inline std::unordered_map<size_t, Attributes> DEFENDER_TYPE_ATTRIBUTES;

};

class Position {
private:
  int _x;
  int _y;

public:
  Position(int x, int y);

  [[nodiscard]] int get_x() const;
  [[nodiscard]] int get_y() const;

  double distance_to(Position other) const;
  auto operator<=>(const Position &other) const = default;
};

bool is_valid_spawn_position(int x, int y);

bool is_valid_spawn_position(Position pos);

std::vector<Position> get_all_valid_spawn_positions();

class Actor {
private:
  size_t _id;
  size_t _hp;
  size_t _type;
  Position _position;

public:
  Actor(size_t id, size_t hp, size_t type, Position pos);
  size_t get_id() const;
  size_t get_hp() const;
  size_t get_type() const;
  Position get_position() const;
};

class Attacker : public Actor {
public:
  Attacker(size_t id, size_t hp, size_t type, Position pos);
};

class Defender : public Actor {
public:
  Defender(size_t id, size_t hp, size_t type, Position pos);
};

class State {
public:
  State(std::vector<Attacker> attackers, std::vector<Defender> defenders,
        size_t no_of_coins_left, size_t turn_no);

  const std::vector<Attacker> &get_attackers() const;
  const std::vector<Defender> &get_defenders() const;
  size_t get_turn_no() const;
  size_t get_coins_left() const;

private:
  size_t _turn_no;
  size_t _no_of_coins_left;
  std::vector<Attacker> _attackers;
  std::vector<Defender> _defenders;
};

class Game {
  std::unordered_map<size_t, size_t> _player_set_targets;
  std::vector<std::pair<size_t, Position>> _spawn_postions;
  std::set<Position> _already_spawned_positions;
  std::ostringstream _logr;

public:
  Game();
  void spawn_attacker(size_t id, Position pos);
  bool already_spawned_at_position(Position pos);
  void set_target(size_t attacker_id, size_t defender_id);
  void set_target(const Attacker &attacker, const Defender &defender);
  std::ostringstream &logr();

  const std::unordered_map<size_t, size_t> &get_player_set_targets() const;
  const std::vector<std::pair<size_t, Position>> &get_spawn_positions() const;
  const std::set<Position> &get_already_spawned_positions() const;
};

class Map {
public:
  Map(std::vector<std::vector<int>> map_as_grid);

  static Map get(std::istream &stream);

  [[nodiscard]] std::vector<Defender> spawn_defenders() const;

  static inline size_t no_of_rows;
  static inline size_t no_of_cols;

private:
  std::vector<std::vector<int>> _grid;
};

Game run(const State &state);

#define logger game.logr()
