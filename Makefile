.PHONY: lint test build release deploy

lint:
	cargo clippy

test:
	cargo test

build:
	cargo build --release

release:
	cp target/release/easy-window-switcher-rs ~/bin

deploy: build release