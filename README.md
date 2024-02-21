# URBR Federated Inference

## Prerequisite
* cargo
  * wasm-pack

## Run Test (not yet)
```sh
cargo test
```

## Run Benchmark
```sh
cargo bench
```

## Build WASM (Front)
```sh
cd bindings/wasm
wasm-pack build --target web --release
```

## Build Go (Back)
```sh
cd bindings/go
cargo build --release
```
