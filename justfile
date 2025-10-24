@clean:
    rm -rf examples/out/**/*.webp

@examples: clean
    cargo r --example overlay
    cargo r --example ring
    cargo r --example custom

@doc:
    cargo doc
    miniserve target/doc

@wasm:
    cd examples/wasm && bun dev