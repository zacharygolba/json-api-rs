#!/usr/bin/env bash

set -e
source scripts/setup.sh

echo ""

run rustup install stable beta $NIGHTLY
run rustup default $DEFAULT_TOOLCHAIN

if ! has_plugin clippy; then
  run cargo +$NIGHTLY install clippy
fi

if ! has_plugin fmt; then
  run cargo +$NIGHTLY install rustfmt-nightly
fi

run cargo update
run cargo build

run_plugin $NIGHTLY fmt --all -- --write-mode diff

if [ $DEFAULT_TOOLCHAIN == $NIGHTLY ]; then
  run_plugin $NIGHTLY clippy --all
  run cargo test --all --all-features
else
  run_plugin $NIGHTLY clippy
  run cargo test
fi
