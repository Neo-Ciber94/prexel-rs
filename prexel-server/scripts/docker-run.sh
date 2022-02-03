#!/bin/bash

$port=${1:-5000}

cargo +nightly build -Z unstable-options --out-dir=./target/release --release

docker build -t prexel-server:latest .

docker run -it -dp $port:$port -- prexel-server:latest