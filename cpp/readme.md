cpp 只能使用clang-15 编译 wasm

windows 和 linux 均可

使用emcc编译的wasm不兼容


testc1_game Error: thread 'main' panicked at 'failed to instantiate wasm module: Instantiation("Export _Znam not found")', crates/zkwasm/src/runtime/wasmi_interpreter.rs:84:51
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace