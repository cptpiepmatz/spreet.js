def main [] {
  build
  test
}

export def "build" [] {
  build wasm
  build postbuild
}

export def "build wasm" [] {
  ^cargo build --target wasm32-unknown-unknown --release
  ^wasm-bindgen target/wasm32-unknown-unknown/release/spreet_js_imports.wasm --out-dir imports --target experimental-nodejs-module
}

export def "build postbuild" [] {
  ^cargo run --bin postbuild --features postbuild
}

export def "test" [] {
  ^deno test --allow-read --allow-write
}
