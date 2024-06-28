#!/bin/sh

cargo build --release
mv target/release/catnip ~/.local/bin/
