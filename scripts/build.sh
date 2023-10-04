#!/bin/zsh

cargo build --release
mkdir -p $HOME/.local/bin
cp target/release/mru $HOME/.local/bin