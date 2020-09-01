#!/usr/bin/env bash

cargo build --example $1
halite --turn-limit 1000 --replay-directory replays/ -vvvvv --width 32 --height 32 "RUST_BACKTRACE=full ./target/debug/examples/$1" "RUST_BACKTRACE=full ./target/debug/examples/$1"
