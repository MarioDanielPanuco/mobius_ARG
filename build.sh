#!/usr/bin/env bash
ar=$1
printf ar
printf "Installing Rust toolchain\n"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
printf "Build\n"

cargo install --locked trunk
trunk serve --release

#cd /dist
#npm build -i
#
