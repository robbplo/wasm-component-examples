#! /usr/bin/env bash

wkg wit fetch
go tool wit-bindgen-go generate --world main --out gen ./wit
tinygo build --target=wasip2 --wit-package ./wit --wit-world main -o nth_prime_number.wasm