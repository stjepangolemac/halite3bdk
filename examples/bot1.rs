use halite3bdk::game::Game;

fn main() {
    let mut game = Game::init("bot1");

    loop {
        game.next_turn();
        game.end_turn();
    }
}
