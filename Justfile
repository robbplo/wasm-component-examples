
build-rust:
  cd rust/concat-text && . ./build.sh
  cd rust/http-request && . ./build.sh
  cd rust/nth-prime-number && . ./build.sh
  cp rust/concat-text/concat_text.wasm rust/http-request/http_request.wasm rust/nth-prime-number/nth_prime_number.wasm ./rust

build-go:
  cd go/concat-text && . ./build.sh
  cd go/http-request && . ./build.sh
  cd go/nth-prime-number && . ./build.sh
  cp go/concat-text/concat_text.wasm go/http-request/http_request.wasm go/nth-prime-number/nth_prime_number.wasm ./go