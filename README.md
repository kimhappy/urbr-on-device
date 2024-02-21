# URBR Federated Inference

## Prerequisite
* cargo (command depends on your platform)
  * wasm-pack (with `cargo install wasm-pack`)

## Run Benchmark
```sh
cargo bench
```

## Build WASM (Front)
* 사용 예시는 [bindings/wasm/test.html](bindings/wasm/test.html) 참고
```sh
cd bindings/wasm
wasm-pack build --target web --release
```

## Build Go (Back)
* 사용 예시는 [bindings/wasm/test.go](bindings/wasm/test.go) 참고
  * 시작 부분 `-L./target/release` 주석은 경로 맞춰서 수정
  * `/* */` 부분 주석이 아니라 링커가 보는 코드임
* 서버와 같은 운영체제 (Windows, MacOS, Linux) 에서 빌드해야 함

```sh
cd bindings/go
cargo build --release
```
