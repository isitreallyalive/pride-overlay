@clean:
    rm -rf examples/out/**/*.webp

@examples: clean
    cargo r --example overlay
    cargo r --example ring
    cargo r --example custom

@doc:
    cargo doc
    miniserve target/doc

wasm: wasm-build wasm-dev

@wasm-build:
    wasm-pack build

@wasm-dev:
    cd examples/wasm && bun dev

@wasm-doc:
    cargo doc --target wasm32-unknown-unknown
    miniserve target/wasm32-unknown-unknown/doc