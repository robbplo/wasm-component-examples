#! /usr/bin/env bash

# example how to run the wasm component and pretty print the json response
wasmtime run --invoke 'run("posts")' -S http http_request.wasm | sed 's/^ok(//;s/)$//' | jq -r . | jq .
