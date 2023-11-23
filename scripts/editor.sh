#!/bin/sh

set -e

cargo build
cargo build --release

godot --editor --path ./godot --rendering-driver opengl3
