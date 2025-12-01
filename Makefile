.PHONY: build
build:
	dx bundle --release
	cp -r target/dx/empc/release/web .

.PHONY: format
format:
	cargo fmt
	dx fmt

.PHONY: check
check:
	cargo clippy -- -D warnings
	cargo clippy -F server -- -D warnings

.PHONY: ci
ci:
	make format
	make check
