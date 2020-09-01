use halite3bdk::direction::Direction;
use halite3bdk::game::Game;
use halite3bdk::random::dice_usize;
use std::collections::HashMap;

enum ShipState {
    Mining,
    Returning,
}

fn main() {
    let mut game = Game::init("bot1");

    let mut ship_state: HashMap<u32, ShipState> = HashMap::new();

    loop {
        game.next_turn();

        let mut cmds = vec![];
        let halite = game.my_halite();
        let shipyard = game.my_shipyard();
        let ships = game.my_ships();

        // We spawn some ships
        let shipyard_occupied = game.map.is_occupied(&shipyard.position);
        if halite > 1000 && !shipyard_occupied {
            let cmd = shipyard.spawn();
            cmds.push(cmd);
        }

        // We move all ships to the north, why not
        ships.iter().for_each(|ship| {
            match ship_state.get(&ship.id).unwrap_or(&ShipState::Mining) {
                ShipState::Mining => {
                    if ship.halite < 1000 {
                        let tile_halite = game.map.get_cell(&ship.position).halite;
                        let tile_is_worth = tile_halite > 1000 / 10;

                        if !tile_is_worth {
                            let directions = Direction::all();
                            let chosen_dir = directions[dice_usize(4)];

                            let future_position = &ship.position.with_direction(&chosen_dir);
                            let is_safe = !game.map.is_occupied(future_position);

                            if is_safe {
                                game.map.set_occupied(future_position);
                                let cmd = ship.r#move(chosen_dir);
                                cmds.push(cmd);
                            }
                        }
                    } else {
                        // ship is full, start returning
                        ship_state.insert(ship.id, ShipState::Returning);
                    }
                }
                ShipState::Returning => {
                    let destination = &shipyard.position;

                    if &ship.position != destination {
                        let next_directions = game.map.direction_to(&ship.position, destination);
                        let direction = next_directions.first();

                        match direction {
                            Some(dir) => {
                                let future_position = &ship.position.with_direction(&dir);
                                let is_safe = !game.map.is_occupied(future_position);

                                if is_safe {
                                    game.map.set_occupied(future_position);
                                    let cmd = ship.r#move(*dir);
                                    cmds.push(cmd);
                                }
                            }
                            None => (),
                        }
                    } else {
                        // ship is home, now go back to mining
                        ship_state.insert(ship.id, ShipState::Mining);
                    }
                }
            }
        });

        game.register_commands(cmds);
        game.end_turn();
    }
}
