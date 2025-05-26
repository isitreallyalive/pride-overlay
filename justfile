wasm-dev:
    wasm-pack build --out-dir examples/wasm/pride-overlay --dev
    cd examples/wasm && bun i && bun dev