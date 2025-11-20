.PHONY: build
build:
	dx bundle

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
	make build
	make check
