# URBR Federated Inference

## Prerequisite
* cargo (command depends on your platform)
  * wasm-pack (with `cargo install wasm-pack`)

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
* 서버와 같은 운영체제 (Windows, MacOS, Linux) 에서 빌드할 것!

```sh
cd bindings/go
cargo build --release
```
