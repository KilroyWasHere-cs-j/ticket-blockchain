#!/bin/bash

echo "Building..."
cargo build

echo "Running with heaptrack"

heaptrack ./target/debug/tblock 1 2
