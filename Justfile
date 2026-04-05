build-wasm-kernel:
    @wasm-pack build crates/wasm \
        --release \
        --target bundler \
        --out-dir ../../web/frontend/src/lib/cassida \
        --out-name kernel
    @cargo test --features=ts export_bindings

[working-directory('web/frontend')]
serve-frontend-dev:
    npm install
    npm run dev

server:
    cargo run --bin server

[parallel]
serve-dev: server serve-frontend-dev

[working-directory('web/frontend')]
build-frontend:
    @VITE_USE_WASM=true npm run build

[working-directory('web/frontend')]
serve-static: build-wasm-kernel build-frontend
    @python -m http.server -d build

profile-integration-tests:
    cargo flamegraph -p symbolics --profile profiling --unit-test -- tests::integration_tests::test_full_processing_chain

visualize-pattern-program *args:
    @cargo run -q --bin visualize-pattern-program -- {{ args }}
