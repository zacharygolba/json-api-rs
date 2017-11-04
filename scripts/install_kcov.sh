#!/usr/bin/env bash

set -e

PROJECT=$(pwd)

cd /tmp

apt-get update -y
apt-get install -y \
  binutils-dev \
  cmake \
  gcc \
  libcurl4-openssl-dev \
  libelf-dev libdw-dev \
  libiberty-dev

git clone https://github.com/SimonKagstrom/kcov.git
cd kcov

mkdir build
cd build

cmake ..
make
make install

cd $PROJECT
