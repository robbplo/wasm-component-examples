
build-rust:
  cd rust/concat-text && . ./build.sh
  cp rust/concat-text/concat_text.wasm dist/rust_concat_text.wasm
  cd rust/http-request && . ./build.sh
  cp rust/http-request/http_request.wasm dist/rust_http_request.wasm
  cd rust/nth-prime-number && . ./build.sh
  cp rust/nth-prime-number/nth_prime_number.wasm dist/rust_nth_prime_number.wasm

build-go:
  cd go/concat-text && . ./build.sh
  cp go/concat-text/concat_text.wasm dist/go_concat_text.wasm
  cd go/http-request && . ./build.sh
  cp go/http-request/http_request.wasm dist/go_http_request.wasm
  cd go/nth-prime-number && . ./build.sh
  cp go/nth-prime-number/nth_prime_number.wasm dist/go_nth_prime_number.wasm
