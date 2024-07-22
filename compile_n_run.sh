#!/bin/bash

# Compile the project
echo "Compiling DFEXE..."
cargo build --release --target aarch64-apple-darwin

# Check if compilation was successful
if [ $? -ne 0 ]; then
    echo "Compilation failed. Please check the error messages above."
    exit 1
fi

# Run the compiled binary
echo "Running DFEXE..."
./target/aarch64-apple-darwin/release/dfexe