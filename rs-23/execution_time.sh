#!/bin/bash
#
# This script will run all solutions and measure the time spent.
# The goal is to stay as efficient as possible!

DIR=./src/bin/

cargo clean
cargo build --release --bins

time find $DIR -type f -printf '%f\n' | grep -E -o "day[0-9]+b?" | xargs -n 1 cargo run --release --bin
