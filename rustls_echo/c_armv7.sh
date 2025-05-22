#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <bin_name>"
    exit 1
fi
bin_name=$1
cross build --release --target armv7-unknown-linux-gnueabihf --bin $bin_name
