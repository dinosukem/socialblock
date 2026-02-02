build:
    cargo build --release

fmt:
    cargo fmt --all

lint:
    cargo clippy --all -- -D warnings

ci: fmt lint

cp:
    cp -f ./target/release/socialblock ./socialblock && chmod +x ./socialblock
