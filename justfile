dev:
    cargo run

wasm-build:
    bevy build web --bundle

wasm-check:
    cargo c --target wasm32-unknown-unknown
