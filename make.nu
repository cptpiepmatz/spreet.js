def main [] {
  build
  test
}

export def "build" [] {
  build wasm
  build postbuild
}

export def "build wasm" [] {
  ^cargo build --target wasm32-wasip1 --release
}

export def "build postbuild" [] {
  ^cargo run --bin postbuild
}

export def "test" [] {
  ^npm install --no-save ts-node
  ^node --loader ts-node/esm --test tests\plugin.test.mts
}
