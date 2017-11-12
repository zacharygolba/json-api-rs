#!/usr/bin/env bash

set -e
source scripts/setup.sh

echo ""

run rustup install stable beta $NIGHTLY
run rustup default $DEFAULT_TOOLCHAIN

if ! has_plugin clippy; then
  run cargo +$NIGHTLY install clippy --vers 0.0.170
fi

if ! has_plugin fmt; then
  run cargo +$NIGHTLY install rustfmt-nightly --vers 0.2.15
fi

run cargo update
run cargo build

if [ $DEFAULT_TOOLCHAIN == $NIGHTLY ]; then
  run_plugin $NIGHTLY clippy --all
  run_plugin $NIGHTLY fmt --all -- --write-mode diff
  run cargo test --all --all-features
else
  run_plugin $NIGHTLY clippy
  run_plugin $NIGHTLY fmt -- --write-mode diff
  run cargo test
fi

if [ "$CIRCLECI" == "true" ]; then
  if ! [ -f /usr/local/bin/kcov ]; then
    run scripts/install_kcov.sh
  fi

  if ! has_plugin kcov; then
    run cargo install cargo-kcov
  fi

  if [ $DEFAULT_TOOLCHAIN == $NIGHTLY ]; then
    run_plugin $NIGHTLY kcov --all --lib --no-clean-rebuild
  else
    run_plugin stable kcov --lib --no-clean-rebuild
  fi
fi
