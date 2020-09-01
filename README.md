# halite3bdk

A Rust Bot Development Kit for Halite III.

## Features

- stdin / stdout communication
- file logger
- very simple to use

# Usage

A minimal example:

```
use halite3bdk::game::Game;

fn main() {
    let mut game = Game::init("bot1");

    loop {
        game.next_turn();

        // your logic goes here

        game.end_turn();
    }
}
```

For more complex behaviors check out the `examples/` dir.
