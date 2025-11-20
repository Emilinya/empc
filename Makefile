.PHONY: build
build:
	dx bundle

.PHONY: format
format:
	cargo fmt
	dx fmt

.PHONY: check
check:
	cargo clippy

.PHONY: ci
ci:
	make format
	make build
	make check