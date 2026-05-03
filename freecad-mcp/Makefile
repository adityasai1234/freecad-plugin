.PHONY: build test check run install clean

build:
	cargo build --release

test:
	cargo test

check:
	cargo check && cargo clippy --all-targets -- -D warnings

run:
	RUST_LOG=debug cargo run

install:
	cp target/release/freecad-mcp ~/.local/bin/

clean:
	cargo clean
