#include "player_code.h"

// This initial code is well commented and serves as a small tutorial for game
// APIs, for more information you can refer to the documentation

// This is the function player has to fill
// You can define any new functions here that you want
Game run(const State &state) {

  // Always start by instantiating a Game class object
  Game game;

  size_t remaining_coins = state.get_coins_left();

  game.logr() << "TURN " << state.get_turn_no() << " LOGS:";

  // Get all the attackers and defenders in the game and store it
  const std::vector<Attacker> &attackers = state.get_attackers();
  const std::vector<Defender> &defenders = state.get_defenders();

  // The function get_all_valid_spawn_positions() is a helper which will give us
  // the list of valid spawn positions in map.
  // If the position  we're spawning is not one of these, the player will be
  // penalized by deducting the spawn cost but not spawning the attacker
  std::vector<Position> all_valid_spawn_positions =
      get_all_valid_spawn_positions();

  // Lets say I want to spawn an attacker of each of the type in one turn
  // and I want to use the all_valid_spawn_positions list as well. In order to
  // keep traack of the last index in the list that we spawned at, we can use a
  // static variable in c++

  static int last_spawned = 0;

  // If there's no defenders left,we can stop spawning and save up on coins,
  // which are important for boosting game score
  if (!defenders.empty()) {
    for (size_t type_id = 1; type_id <= Constants::NO_OF_ATTACKER_TYPES;
         type_id++) {
      // Spawn the attacker of type_id at position
      // all_valid_spawn_positions[last_spawned]

      // There are two cases when you might be panalized
      //    - Spawning at invalid position
      //    - Spawning at position where you have already spawned one attacker
      //    in the same turn
      //
      // We have provided helpers to check just that

      // game class will keep track of all your spawned positions for you and
      // provides a helper method called already_spawned_at_position(Position)
      // to check if you already spawned in the position

      // Mostly a good practice to check with these two helpers before spawning,
      // to save up on accidental penalties
      if (is_valid_spawn_position(all_valid_spawn_positions[last_spawned]) &&
          !game.already_spawned_at_position(
              all_valid_spawn_positions[last_spawned])) {
        // If lets say you had run out of coins left, the game will just ignore
        // the spawn
        game.spawn_attacker(type_id, all_valid_spawn_positions[last_spawned]);

        // This has the starting attributes for the attacker we are about to
        // spawn
        // For full information about the Attributes class refer the
        // documentation
        Attributes attackers_attributes =
            Constants::ATTACKER_TYPE_ATTRIBUTES.at(type_id);

        // You can use the logger we provide to show log messages in the
        // rendered game
        game.logr() << "(" << attackers_attributes.hp << ","
                    << attackers_attributes.attack_power
                    << ") to be spawned at Position("
                    << all_valid_spawn_positions[last_spawned].get_x() << ","
                    << all_valid_spawn_positions[last_spawned].get_y() << ")"
                    << '\n';
        (last_spawned += 1) %= all_valid_spawn_positions.size();
      }
    }
  }

  // Now lets say you always want to set the target for the attackers[0] to
  // defenders[0]
  // To do that you do
  if (!attackers.empty() && !defenders.empty()) {
    // check if they are empty beforehand to be safe from unexpected errors
    game.set_target(attackers.front(), defenders.front());
  }

  // Lets log all the spawned positions for this turn
  for (auto &[type_id, pos] : game.get_spawn_positions()) {
    // you can use logger macro as well, which is an alias for game.logr()
    logger << "Type " << type_id << " at Position (" << pos.get_x() << ","
           << pos.get_y() << ")\n";
  }

  // always  return the game object
  return game;
}
