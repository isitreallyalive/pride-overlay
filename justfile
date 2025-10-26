@clean:
    rm -rf examples/out/**/*.webp

@examples *args: clean
    cargo r --example custom {{args}}
    cargo r --example overlay {{args}}
    cargo r --example ring {{args}}

@doc:
    cargo doc
    miniserve target/doc

@clippy:
    cargo clippy
    cargo clippy --target wasm32-unknown-unknown

wasm: wasm-build wasm-dev

@wasm-build *args:
    wasm-pack build {{args}}

@wasm-dev:
    cd examples/wasm && bun dev

@wasm-doc:
    cargo doc --target wasm32-unknown-unknown
    miniserve target/wasm32-unknown-unknown/doc