#! /usr/bin/env bash

# example how to run the wasm component and pretty print the json response
wasmtime run --invoke 'run("photos")' -S http http_request.wasm | sed 's/^ok(//;s/)$//' | jq -r . | jq .
