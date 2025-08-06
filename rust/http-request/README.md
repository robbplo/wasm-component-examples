# Http request

Http component that calls the typicode api. The actual sending of the request is done via the wasi:http/outgoing-handler interface.

## Building

requires rust with target wasm32-wasip2 installed and the `wkg` tool for fetching the wit dependencies.

building can be done via:

```sh
. build.sh
```

## Running

there is an example script how to run the component:

```sh
. run.sh
```
