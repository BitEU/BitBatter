#!/bin/bash

echo "Building Baseball TUI..."
echo

echo "Running unit tests..."
echo
cargo test

if [ $? -ne 0 ]; then
    echo
    echo "Tests failed! Please fix the failing tests before building."
    exit 1
fi

echo
echo "All tests passed!"
echo
echo "Building release version..."
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
echo "target/release/BitBatter"
echo
echo "To run the game:"
echo "  cargo run --release"
echo "or"
echo "  ./target/release/BitBatter"
echo
