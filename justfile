@clean:
    rm -rf examples/*.gif examples/*.webp examples/*.png

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

wasm-build profile="dev" *args: wasm-clean
    cd wasm && bun esbuild.ts
    wasm-pack build --target web --no-default-features --features default-formats {{args}}
    cp wasm/squoosh/*.wasm pkg

@wasm:
    just wasm-build
    miniserve --index examples/web.html