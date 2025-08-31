build:
    cargo build --release

install: build
    cp target/release/convmit ~/.local/bin/convmit
