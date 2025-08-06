
build-rust:
  cd rust/concat-text && . ./build.sh
  cd rust/http-request && . ./build.sh
  cd rust/nth-prime-number && . ./build.sh
  cp rust/concat-text/concat_text.wasm rust/http-request/http_request.wasm rust/nth-prime-number/nth_prime_number.wasm .
