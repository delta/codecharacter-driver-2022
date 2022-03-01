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

# This initial code is well commented and serves as a small tutorial for game
# APIs, for more information you can refer to the documentation

# This is the function player has to fill
# You can define any new functions here that you want


last_spawned = 0

def run(state: State) -> Game:
    global last_spawned
    # Always start by instantiating a Game class object
    game = Game()

    remaining_coins = state.no_of_coins_left

    game.log("TURN {} LOGS:\n".format(state.turn_no))
    
    # Get all the attackers and defenders in the game and store it
    attackers = state.attackers
    defenders = state.defenders
    
    # The function get_all_valid_spawn_positions() is a helper which will give us
    # the list of valid spawn positions in map.
    # If the position  we're spawning is not one of these, the player will be
    # penalized by deducting the spawn cost but not spawning the attacker
    all_valid_spawn_positions = get_all_valid_spawn_positions()
    
    # If there's no defenders left,we can stop spawning and save up on coins,
    # which are important for boosting game score
    if len(defenders) != 0:
        for type_id in range(1,Constants.NO_OF_ATTACKER_TYPES+1):
            
            # Spawn the attacker of type_id at position
            # all_valid_spawn_positions[last_spawned]

            # There are two cases when you might be panalized
            #    - Spawning at invalid position
            #    - Spawning at position where you have already spawned one attacker
            #    in the same turn
             
            # We have provided helpers to check just that

            # game class will keep track of all your spawned positions for you and
            # provides a helper method called is_already_spawned_at_position(Position)
            # to check if you already spawned in the position

            # Mostly a good practice to check with these two helpers before spawning,
            # to save up on accidental penalties

            if is_valid_spawn_position(all_valid_spawn_positions[last_spawned]) and\
                    not game.is_already_spawned_at_position(all_valid_spawn_positions[last_spawned]):
                        
                # If lets say you had run out of coins left, the game will just ignore
                # the spawn
                game.spawn_attacker(type_id, all_valid_spawn_positions[last_spawned])
                
                # You can use the logger we provide to show log messages in the
                # rendered game
                # For full information about the AttackerType class refer the
                # documentation
                attackers_attributes: AttackerType = Constants.ATTACKER_TYPE_ATTRIBUTES[type_id]

                # You can use the logger we provide to show log messages in the
                # rendered game
                pos = all_valid_spawn_positions[last_spawned]
                game.log("To be spawned at Position({},{})\n".format(pos.x,pos.y))

                last_spawned = last_spawned + 1
                last_spawned = last_spawned % len(all_valid_spawn_positions)
                
    #Now lets say you always want to set the target for the attackers[0] to
    #defenders[0]
    #To do that you do
    if len(attackers)!=0 and len(defenders)!=0:
        game.set_target(attackers[0].id,defenders[0].id)
        
    #Lets log all the spawned positions for this turn
    for type_id, pos in game.spawn_positions:
        game.log("Type {} at Position ({},{})".
            format(type_id, pos.x, pos.y))

    #always  return the game object
    return game

