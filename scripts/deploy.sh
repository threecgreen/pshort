#!/bin/bash
cargo build --release
cp ./target/release/pshort $HOME/.local/bin
