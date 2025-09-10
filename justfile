build:
    cargo build --release

install: build
    cp target/release/convmit ~/.local/bin/convmit

format:
    cargo fmt --tests

lint:
    cargo clippy

lint-fix:
    cargo clippy --fix --allow-dirty

