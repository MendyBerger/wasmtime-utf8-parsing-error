# UTF8 parsing error - wasmtime

The error I'm getting is:
```
rror: error while executing at wasm backtrace:
    0: 0x1596ef - wit-component:shim!indirect-$root-my-func
    1:  0x4ab - <unknown>!run
note: using the `WASMTIME_BACKTRACE_DETAILS=1` environment variable may show more debugging information

Caused by:
    invalid utf-8 sequence of 1 bytes from index 0
```

#### To replicate the issue:

```bash
cd wasm
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/wasm.wasm -o out.wasm
cd ../runtime
cargo run
```
