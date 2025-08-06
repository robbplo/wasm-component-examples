#! /usr/bin/env bash

cargo build --release --target wasm32-wasip2
cp ../target/wasm32-wasip2/release/nth_prime_number.wasm .
