# WASM Bindings for rs-graph's MaxFlow

## Build

```sh
docker build -t maxflow-wasm . &&
docker run -d -v ${PWD}:/usr/src/maxflow-wasm maxflow-wasm
```
