use halite3bdk::direction::Direction;
use halite3bdk::game::Game;
use halite3bdk::random::dice_usize;

fn main() {
    let mut game = Game::init("bot1");

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

            // we don't do anything if the tile is worth it, we keep mining
        });

        game.register_commands(cmds);
        game.end_turn();
    }
}
