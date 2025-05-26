use cfg_aliases::cfg_aliases;

fn main() {
    // make it easier to gate wasm-specific functionality
    cfg_aliases! {
        wasm: { target_arch = "wasm32" }
    }
}
