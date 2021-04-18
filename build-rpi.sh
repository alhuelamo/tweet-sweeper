#!/usr/bin/env bash

DIR=$(pwd)

if ! [[ $(command -v aarch64-linux-gnu-gcc) ]] ; then
    echo 'Make sure to install aarch64 toolchain (aarch64-linux-gnu-gcc)'
    echo 'In Ubuntu: sudo apt install gcc-aarch64-linux-gnu'
    exit 1
fi

# Build OpenSSL
cd /tmp
rm -rf openssl-1.1.1k

wget https://www.openssl.org/source/openssl-1.1.1k.tar.gz
tar xzf openssl-1.1.1k.tar.gz
export MACHINE=aarch64
export ARCH=aarch64
export CC=aarch64-linux-gnu-gcc
cd openssl-1.1.1k && ./config shared && make

export OPENSSL_LIB_DIR=/tmp/openssl-1.1.1k/
export OPENSSL_INCLUDE_DIR=/tmp/openssl-1.1.1k/include

cd $DIR

# Build project
cargo build --release --target=aarch64-unknown-linux-gnu
