@clean:
    rm -rf examples/*.gif examples/*.webp

@doc:
    cargo doc
    miniserve target/doc

@examples *args:
    cargo r --example overlay {{args}}
    just wasm-example

@flame example *args:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --example {{example}} --output examples/{{example}}.svg {{args}}
    rm perf.data

@wasm-clean:
    rm -rf pkg

@wasm-build: wasm-clean
    wasm-pack build --target web --no-default-features --features all-formats

@wasm: wasm-build
    miniserve --index examples/web.html