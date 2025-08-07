# Concat text

Http component that calls the typicode api. The actual sending of the request is done via the wasi:http/outgoing-handler interface.

## prerequirements
* Install `tinygo` [install tinygo](https://tinygo.org/getting-started/install/)
* Install `golang` version 1.24 or greater
* Requires the `wkg` tool
* Run `go mod download` to download the dependencies

## Building

building can be done via:

```sh
. build.sh
```

## Running

there is an example script how to run the component:

```sh
. run.sh
```
