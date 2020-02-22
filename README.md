# asc-in-rust
A demo of running wasm created by AssemblyScript in Rust using wasmer.

## Quick Start

### Install wasm32 target

To compile Rust into wasm, add target by following command:

```
$ rustup target add wasm32-unknown-unknown
```

### Compile Rust into wasm

```
$ rustc rust-demo/src/wasm/add.rs -o rust-demo/src/wasm/add.wasm --target=wasm32-unknown-unknown
```

### Obvious "Hello world"

```
$ cd rust-demo && cargo run
```

## Run wasm compiled in Ts in Rust

### Compile wasm from typescript (using assemblyscript)

```
$ cd ts-demo
$ npm install
$ npm run asbuild
```

### Copy wasm file

```
$ cd ../
$ cp ts-demo/build/optimized.wasm rust-demo/examples/wasm/add-ts.wasm
```

### Run in Rust

```
$ cd rust-demo
$ WASM_PATH="wasm/add-ts.wasm" cargo run --example asc
```
