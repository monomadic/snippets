/*
$ cargo install wasm-pack
# or
$ nix-shell --packages wasm-pack rustup
$ rustup install stable

$ cargo new --lib hello-wasm

cargo.toml:
    [lib]
    crate-type = ["cdylib"]

    [dependencies]
    wasm-bindgen = "0.2"

$ wasm-pack build --target web --no-typescript --out-dir public hello.wasm
# note the --target web is if you aren't using npm (noob package manager)

# to strip the wasm binary for release
$ wasm-gc target/wasm32-unknown-unknown/debug/wasm_sample.wasm
*/

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str); // bind external function
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
