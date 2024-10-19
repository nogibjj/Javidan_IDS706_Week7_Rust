format:
	rustfmt src/main.rs
	rustfmt src/lib.rs
check:
	cargo check
test:
	cargo test