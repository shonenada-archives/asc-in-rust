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
$ rustc demo/src/wasm/add.rs -o demo/src/wasm/add.wasm --target=wasm32-unknown-unknown
```

### Obvious "Hello world"

```
$ cd demo && cargo run
```
