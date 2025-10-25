#!/bin/bash

echo "Building Baseball TUI..."
echo

cargo build --release

if [ $? -ne 0 ]; then
    echo
    echo "Build failed! Make sure Rust is installed:"
    echo "https://rustup.rs/"
    exit 1
fi

echo
echo "Build successful!"
echo
echo "Executable location:"
echo "target/release/terminalbball"
echo
echo "To run the game:"
echo "  cargo run --release"
echo "or"
echo "  ./target/release/terminalbball"
echo
