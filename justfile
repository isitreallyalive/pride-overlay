@doc:
    cargo doc
    miniserve target/doc

@flame example *args:
    CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --example {{example}} --output examples/{{example}}.svg {{args}}
    rm perf.data