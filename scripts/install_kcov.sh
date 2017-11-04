#!/usr/bin/env bash

set -e

PROJECT=$(pwd)

cd /tmp

git clone https://github.com/SimonKagstrom/kcov.git
cd kcov

mkdir build
cd build

cmake ..
make
make install

cd $PROJECT
