build:
    cargo build --release && cp -f ./target/release/socialblock ./socialblock && chmod +x ./socialblock

fmt:
    cargo fmt --all

lint:
    cargo clippy --all -- -D warnings

ci: fmt lint
