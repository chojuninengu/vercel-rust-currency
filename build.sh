#!/bin/bash
set -e

# Set up directories
export RUSTUP_HOME="/tmp/rustup"
export CARGO_HOME="/tmp/cargo"
mkdir -p $RUSTUP_HOME $CARGO_HOME

# Install Rust without prompts
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path

# Add cargo to PATH
source "$CARGO_HOME/env"

# Build the project
cargo build --release 