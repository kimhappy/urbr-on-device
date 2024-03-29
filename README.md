# 2024 HYUNDAI MOBIS HACKATHON - URBR Federated Inference

## Prerequisite
* cargo: command depends on your platform
  * wasm-pack: `cargo install wasm-pack`

## Get Git Submodule
```sh
git submodule update --init --recursive
```

## Make Parameter Files (for src/data.rs, parameter_for_tch.pt)
```sh
python3 make_data.py
```

## Run Test
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
* 사용 예시는 [bindings/wasm/test.html](bindings/wasm/test.html) 참고

## Build Go (Back)
```sh
cd bindings/go
cargo build --release
```
* 사용 예시는 [bindings/wasm/test.go](bindings/wasm/test.go) 참고
  * 시작 부분 `-L./target/release` 주석은 경로 맞춰서 수정
  * `/* */` 부분은 주석이 아니라 링커가 보는 코드임
  * 라이브러리 못찾으면 `LD_LIBRARY_PATH=$LD_LIBRARY_PATH:...`
* 서버와 같은 운영체제 (Windows, MacOS, Linux) 에서 빌드해야 함
