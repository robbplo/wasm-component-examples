#! /usr/bin/env bash

# example how to run the wasm component and pretty print the json response
wasmtime run --invoke 'calculate(255)' nth_prime_number.wasm | sed 's/^ok(//;s/)$//'
