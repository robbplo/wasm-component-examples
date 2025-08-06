#! /usr/bin/env bash

# example how to run the wasm component and pretty print the json response
wasmtime run --invoke 'concat("Hello", "World")' concat_text.wasm
