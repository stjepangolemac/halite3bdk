use halite3sdk::logger::{error, info, init};

fn main() {
    init();

    info("hello world");
    error("what is this");
}
