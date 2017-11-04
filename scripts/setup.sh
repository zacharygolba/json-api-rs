#!/usr/bin/env bash

set -e

CARGO_BIN="$CARGO_HOME/bin"
OUTPUT="$TMPDIR/stderr.log"

EXEC="\u001b[46m\u001b[30m  EXEC  \u001b[39m\u001b[49m"
FAIL="\u001b[41m\u001b[30m  FAIL  \u001b[39m\u001b[49m"
PASS="\u001b[42m\u001b[30m  PASS  \u001b[39m\u001b[49m"
SKIP="\u001b[43m\u001b[30m  SKIP  \u001b[39m\u001b[49m"

function run() {
  local cmd="$@"

  echo -en "$EXEC $cmd"

  if $cmd 1>$OUTPUT 2>$OUTPUT ; then
    echo -en "\r"
    echo -e "$PASS $cmd"
  else
    echo -en "\r"
    echo -e "$FAIL $cmd"

    cat $OUTPUT

    exit 1
  fi

  rm $OUTPUT
}

function has-plugin() {
  if [ -f "$CARGO_BIN/cargo-$1" ]; then
    true
  else
    false
  fi
}

function run-plugin() {
  local toolchain=$1; shift
  local cmd="cargo +$toolchain $@"

  if [ $toolchain == $DEFAULT_TOOLCHAIN ]; then
    cmd="cargo $@"
  fi

  if has-plugin $1; then
    run $cmd
  else
    echo -e "$SKIP $cmd (not found in $CARGO_BIN)"
  fi
}

case $CIRCLE_NODE_INDEX in
  0) DEFAULT_TOOLCHAIN="stable"
  ;;
  1) DEFAULT_TOOLCHAIN="beta"
  ;;
  *) DEFAULT_TOOLCHAIN=$NIGHTLY
  ;;
esac
