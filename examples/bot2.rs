use halite3bdk::direction::Direction;
use halite3bdk::game::Game;

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
            // I need to change the method name as `move` is a
            // reserved keyword and needs to be invoked like
            // this, for now
            let cmd = ship.r#move(Direction::North);
            cmds.push(cmd);
        });

        game.register_commands(cmds);
        game.end_turn();
    }
}
